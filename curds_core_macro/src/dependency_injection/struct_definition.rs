use super::*;

#[derive(Clone)]
pub struct StructDefinition {
    visibility: Option<Token![pub]>,
    pub name: Ident,
    dependencies: Vec<InjectedDependency>,
    defaults: HashSet<Ident>,
    with_library: bool,
}
impl StructDefinition {
    pub fn parse(input: ParseStream, with_library: bool) -> Result<Self> {
        let defaults: HashSet<Ident> = DefaultedField::parse_defaults(input)?
            .into_iter()
            .collect();
        let visibility: Option<Token![pub]> = input.parse()?;
        input.parse::<Token![struct]>()?;
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let dependencies = InjectedDependency::parse(content, &defaults)?;

        Ok(Self {
            visibility: visibility,
            name: name,
            dependencies: dependencies,
            defaults: defaults,
            with_library: with_library,
        })
    }

    pub fn quote(self) -> TokenStream {
        let struct_tokens = self.clone().struct_tokens();
        let injected_tokens = self.clone().injected_tokens();
        let construct_tokens = self.construct_tokens();

        quote! {
            #struct_tokens
            #injected_tokens
            #construct_tokens
        }
    }
    fn struct_tokens(self) -> TokenStream {
        let visibility = self.visibility;
        let name = self.name;
        let struct_fields = self.dependencies
            .into_iter()
            .map(|dependency| dependency.struct_tokens());

        if self.with_library {
            let library_field = super::library_name();

            quote! {
                #visibility struct #name {
                    #library_field: std::cell::RefCell<std::collections::HashMap<std::any::TypeId, std::rc::Rc<dyn std::any::Any>>>,
                    #(#struct_fields),*
                }
            }
        }
        else {
            quote! {
                #visibility struct #name {
                    #(#struct_fields),*
                }
            }
        }
    }
    fn injected_tokens(self) -> TokenStream {
        let name = self.name;
        let has_constraints = self.dependencies
            .clone()
            .into_iter()
            .any(|field| !field.default);
        let constraint_tokens =
        if has_constraints {
            let mapped_constraints = self.dependencies
                .clone()
                .into_iter()
                .filter_map(|dependency| if !dependency.default { Some(dependency.constraint_tokens()) } else { None });

            quote! { where TProvider : #(#mapped_constraints)+* }
        }
        else {
            quote! {}
        };
        let generator_tokens =
        if has_constraints {
            let mapped_generators = self.dependencies
                .into_iter()
                .filter_map(|dependency| if !dependency.default { Some(dependency.generator_tokens()) } else { None });

            quote! { #(#mapped_generators),* }
        }
        else {
            quote! {}
        };
        
        quote! {
            impl<TProvider> curds_core_abstraction::dependency_injection::Injected<TProvider> for #name
            #constraint_tokens {
                fn inject(provider: &TProvider) -> Self {
                    Self::construct(#generator_tokens)
                }
            }
        }
    }
    fn construct_tokens(self) -> TokenStream {
        let name = self.name;
        let argument_tokens = self.dependencies
            .clone()
            .into_iter()
            .filter_map(|dependency| if !dependency.default { Some(dependency.argument_tokens()) } else { None });
        let initializer_tokens = self.dependencies
            .into_iter()
            .map(|dependency| dependency.initializer_tokens());

        if self.with_library {
            let library_name = super::library_name();

            quote! {
                impl #name {
                    pub fn construct(#(#argument_tokens),*) -> Self {
                        Self {
                            #library_name: std::default::Default::default(),
                            #(#initializer_tokens),*
                        }
                    }
                }
            }
        }
        else {
            quote! {
                impl #name {
                    pub fn construct(#(#argument_tokens),*) -> Self {
                        Self {
                            #(#initializer_tokens),*
                        }
                    }
                }
            }
        }

    }
}