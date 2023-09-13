use std::thread::Scope;

use super::*;

pub struct ScopedItem {
    item: ItemStruct,
}
impl Parse for ScopedItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let item: ItemStruct = input.parse()?;

        Ok(ScopedItem {
            item,
        })
    }
}

impl ScopedItem {
    pub fn quote(self) -> TokenStream {
        let name = &self.item.ident;        
        let initializer_tokens = self.scope_initializers();
        let (impl_generics, type_generics, where_clause) = self.item.generics.split_for_impl();

        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::Scoped for #name #type_generics #where_clause {
                fn scope(&self) -> Self {
                    let mut constructed = Self {
                        #initializer_tokens
                    };

                    constructed
                }
            }
        }
    }
    fn scope_initializers(&self) -> TokenStream {
        let mut initializer_tokens: Vec<TokenStream> = Vec::new();
        match &self.item.fields {
            Fields::Named(named) => {
                for field in &named.named {
                    let name = &field.ident.clone().unwrap();
                    if name.to_string().starts_with(SINGLETON_FIELD_PREFIX) {
                        initializer_tokens.push(quote! { #name: std::default::Default::default() })
                    }
                    else {
                        initializer_tokens.push(quote! { #name: std::clone::Clone::clone(&self.#name) })
                    }
                }
            },
            _ => panic!("Only named fields are supported"),
        }

        quote! {
            #(#initializer_tokens),*
        }
    }


}