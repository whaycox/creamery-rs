use super::*;
use std::collections::HashSet;

pub fn quote_impl(testing_struct_name: &Ident, method: &TraitItemMethod) -> TokenStream {
    let signature = &method.sig;
    let mut input_names: Vec<TokenStream> = Vec::new();
    for input in &method.sig.inputs {
        match input {
            FnArg::Receiver(_) => {},
            FnArg::Typed(ty) => {
                let name = &ty.pat;
                match *ty.ty {
                    Type::Reference(_) => input_names.push(quote! { #name }),
                    _ => input_names.push(quote! { &#name }),
                }
            }
        }
    }

    let compare_input = if input_names.len() > 0 {
        let expected_input_times_field = expected_input_times_field(&method.sig.ident);
        let expected_input_field = expected_input_field(&method.sig.ident);
        let expected_input_failure = format!("the expected inputs for {}::{} were not supplied", testing_struct_name, method.sig.ident);

        quote! {
            let length = self.#expected_input_times_field.lock().unwrap().len();
            for i in 0..length {
                if self.#expected_input_times_field.lock().unwrap()[i] > 0 {
                    self.#expected_input_times_field.lock().unwrap()[i] -= 1;
                    if !(self.#expected_input_field.lock().unwrap()[i])(#(#input_names),*) {
                        panic!(#expected_input_failure);
                    }
                    break;
                }
            }
        }
    }
    else { quote! {} };
    
    let generate_return = match &method.sig.output {
        ReturnType::Default => quote! {},
        ReturnType::Type(_, _) => {
            let returned_times_field = returned_times_field(&method.sig.ident);
            let returned_field = returned_field(&method.sig.ident);
            let default_generator_field = default_generator_field(&method.sig.ident);
            let no_return_failure = format!("a return is necessary for {}::{} but none have been supplied", testing_struct_name, method.sig.ident);

            quote! {
                let length = self.#returned_times_field.lock().unwrap().len();
                for i in 0..length {
                    if self.#returned_times_field.lock().unwrap()[i] > 0 {
                        self.#returned_times_field.lock().unwrap()[i] -= 1;
                        return (self.#returned_field.lock().unwrap()[i])(#(#input_names),*)
                    }
                }
                match &*self.#default_generator_field.lock().unwrap() {
                    Some(generator) => return generator(#(#input_names),*),
                    _ => panic!(#no_return_failure),
                }
            }
        },
    };

    let recorded_calls = recorded_calls_field(&method.sig.ident);
    quote! {
        #signature {
            *self.#recorded_calls.lock().unwrap() += 1;
            #compare_input
            #generate_return
        }
    }
}

pub fn quote_setup_expectations(method: &TraitItemMethod, lifetimes: &HashSet<Ident>) -> Vec<TokenStream> {
    let mut setup_expectations = vec![
        quote_expect_calls(method),
    ];
    let mut input_types: Vec<Box<Type>> = Vec::new();
    for input in &method.sig.inputs {
        match input {
            FnArg::Receiver(_) => {},
            FnArg::Typed(ty) => match &*ty.ty {
                _ => input_types.push(ty.ty.clone()),
            },
        }
    }

    if input_types.len() > 0 {
        setup_expectations.push(quote_store_expected_input(&method.sig.ident, field_input_types(&input_types)));
    }
    match &method.sig.output {
        ReturnType::Default => {},
        ReturnType::Type(_, ty) => {
            setup_expectations.push(quote_default_return(&method, field_input_types(&input_types), field_return_type(&**ty, lifetimes)));
            setup_expectations.push(quote_store_return(&method, field_input_types(&input_types), field_return_type(&**ty, lifetimes)));
        },
    }

    setup_expectations    
}
fn quote_expect_calls(method: &TraitItemMethod) -> TokenStream {
    let expect_calls = expect_calls_method(&method.sig.ident);
    let expected_calls = expected_calls_field(&method.sig.ident);
    quote! {
        pub fn #expect_calls(&self, expected: u32) {
            *self.#expected_calls.lock().unwrap() = Some(expected);
        }
    }
}
fn quote_store_expected_input(ident: &Ident, input_types: Vec<Box<Type>>) -> TokenStream {
    let store_input = store_expected_input_method(ident);
    let expected_input_times_field = expected_input_times_field(ident);
    let expected_input_field = expected_input_field(ident);

    quote! {
        pub fn #store_input<TComparer: 'static + Send + Sync + Fn(#(#input_types),*) -> bool>(&self, comparison: TComparer, times: u32) {
            self.#expected_input_times_field.lock().unwrap().push(times);
            self.#expected_input_field.lock().unwrap().push(std::boxed::Box::new(comparison));
        }
    }
}
fn quote_default_return(method: &TraitItemMethod, input_types: Vec<Box<Type>>, returned_type: Type) -> TokenStream {
    let default_return = default_return_method(&method.sig.ident);
    let default_generator_field = default_generator_field(&method.sig.ident);

    quote! {
        pub fn #default_return<TGenerator: 'static + Send + Sync + Fn(#(#input_types),*) -> #returned_type>(&self, generator: TGenerator) {
            *self.#default_generator_field.lock().unwrap() = Some(std::boxed::Box::new(generator));
        }
    }
}
fn quote_store_return(method: &TraitItemMethod, input_types: Vec<Box<Type>>, returned_type: Type) -> TokenStream {
    let store_return = store_return_method(&method.sig.ident);
    let returned_times_field = returned_times_field(&method.sig.ident);
    let returned_field = returned_field(&method.sig.ident);

    quote! {
        pub fn #store_return<TGenerator: 'static + Send + Sync + Fn(#(#input_types),*) -> #returned_type>(&self, generator: TGenerator, times: u32) {
            self.#returned_times_field.lock().unwrap().push(times);
            self.#returned_field.lock().unwrap().push(std::boxed::Box::new(generator));
        }
    }
}

pub fn quote_assert_expectations(testing_struct_name: &Ident, method: &TraitItemMethod) -> TokenStream {
    let expected_calls = expected_calls_field(&method.sig.ident);
    let recorded_calls = recorded_calls_field(&method.sig.ident);
    let expected_calls_failure = format!("expected {{}} calls to {}::{} but recorded {{}} instead", testing_struct_name, method.sig.ident);
    
    let mut has_inputs = false;
    for input in &method.sig.inputs {
        match input {
            FnArg::Receiver(_) => {},
            FnArg::Typed(_) => {
                has_inputs = true;
                break;
            },
        }
    }
    let input_assert = if has_inputs {
        let expected_input_times_field = expected_input_times_field(&method.sig.ident);
        let expected_input_failure = format!("not all stored input comparisons for {}::{} have been consumed", testing_struct_name, method.sig.ident);

        quote! {
            if self.#expected_input_times_field.lock().unwrap().iter().any(|comparer| *comparer != 0) {
                panic!(#expected_input_failure);
            }
        }
    }
    else { quote! {} };
    let return_assert = match &method.sig.output {
        ReturnType::Default => quote! {},
        ReturnType::Type(_, _) => {
            let returned_times_field = returned_times_field(&method.sig.ident);
            let returned_failure = format!("not all stored returns for {}::{} have been consumed", testing_struct_name, method.sig.ident);

            quote! {
                if self.#returned_times_field.lock().unwrap().iter().any(|generator| *generator != 0) {
                    panic!(#returned_failure);
                }
            }
        },
    };
    
    quote! {
        if self.#expected_calls.lock().unwrap().is_some() && self.#expected_calls.lock().unwrap().unwrap() != *self.#recorded_calls.lock().unwrap() {
            panic!(#expected_calls_failure, self.#expected_calls.lock().unwrap().unwrap(), self.#recorded_calls.lock().unwrap());
        }
        #input_assert
        #return_assert
    }      
}

pub fn quote_reset_expectations(method: &TraitItemMethod) -> TokenStream {
    let expected_calls = expected_calls_field(&method.sig.ident);
    let recorded_calls = recorded_calls_field(&method.sig.ident);

    let mut has_inputs = false;
    for input in &method.sig.inputs {
        match input {
            FnArg::Receiver(_) => {},
            FnArg::Typed(_) => {
                has_inputs = true;
                break;
            },
        }
    }
    let input_reset = if has_inputs {
        let expected_input_times_field = expected_input_times_field(&method.sig.ident);
        let expected_input_field = expected_input_field(&method.sig.ident);

        quote! {
            self.#expected_input_times_field.lock().unwrap().clear();
            self.#expected_input_field.lock().unwrap().clear();
        }
    }
    else { quote! {} };
    let return_reset = match &method.sig.output {
        ReturnType::Default => quote! {},
        ReturnType::Type(_, _) => {
            let returned_times_field = returned_times_field(&method.sig.ident);
            let returned_field = returned_field(&method.sig.ident);

            quote! {
                self.#returned_times_field.lock().unwrap().clear();
                self.#returned_field.lock().unwrap().clear();
            }
        },
    };
    
    quote! {
        *self.#expected_calls.lock().unwrap() = None;
        *self.#recorded_calls.lock().unwrap() = 0;
        #input_reset
        #return_reset
    }      
}