use super::*;

#[derive(Clone)]
pub struct StructDefinition {
    visibility: Option<Token![pub]>,
    pub name: Ident,
    fields: Vec<StructField>,
}
impl Parse for StructDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let defaults: HashSet<Ident> = DefaultedField::parse_defaults(input)?
            .into_iter()
            .collect();
        let visibility: Option<Token![pub]> = input.parse()?;
        input.parse::<Token![struct]>()?;
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let parsed_fields: Punctuated<Field, Token![,]> = content.parse_terminated(Field::parse_named)?;
        let mut fields: Vec<StructField> = Vec::new();
        for field in parsed_fields {
            let field_name = field.ident.clone().unwrap();
            fields.push(StructField::new(field, defaults.contains(&field_name)));
        }

        Ok(Self {
            visibility: visibility,
            name: name,
            fields: fields,
        })
    }
}

impl StructDefinition {
    pub fn dependency_type(&self, name: &Ident) -> Type {
        for field in self.fields.clone() {
            if field.eq(name) {
                return field.ty()
            }
        }
        panic!("no provider found");
    } 

    pub fn quote(self, singletons: Vec<SingletonDependency>) -> TokenStream {
        let struct_tokens = self.clone().struct_tokens(singletons.clone());
        let injected_tokens = self.clone().injected_tokens();
        let construct_tokens = self.construct_tokens(singletons);

        quote! {
            #struct_tokens
            #injected_tokens
            #construct_tokens
        }
    }
    fn struct_tokens(self, singletons: Vec<SingletonDependency>) -> TokenStream {
        let visibility = self.visibility;
        let name = self.name;
        let mut struct_fields: Vec<TokenStream> = self.fields
            .into_iter()
            .map(|field| field.to_token_stream())
            .collect();
        for singleton in singletons {
            struct_fields.push(singleton.field_tokens())
        }

        quote! {
            #visibility struct #name {
                #(#struct_fields),*
            }
        }
    }
    fn injected_tokens(self) -> TokenStream {
        let name = self.name;
        let mapped_constraints: Vec<TokenStream> = self.fields
            .clone()
            .into_iter()
            .filter_map(|dependency| dependency.constraint_tokens())
            .collect();
        let has_constraints = mapped_constraints.len() > 0;
        let constraint_tokens =
        if has_constraints {
            quote! { where TProvider : #(#mapped_constraints)+* }
        }
        else {
            quote! {}
        };
        let generator_tokens = self.fields
            .into_iter()
            .filter_map(|dependency| dependency.generator_tokens());
        
        quote! {
            impl<TProvider> curds_core_abstraction::dependency_injection::Injected<TProvider> for #name
            #constraint_tokens {
                fn inject(provider: &TProvider) -> std::rc::Rc<Self> {
                    Self::construct(#(#generator_tokens),*)
                }
            }
        }
    }
    fn construct_tokens(self, singletons: Vec<SingletonDependency>) -> TokenStream {
        let name = self.name;
        let argument_tokens = self.fields
            .clone()
            .into_iter()
            .filter_map(|dependency| dependency.argument_tokens());
        let mut initializer_tokens: Vec<TokenStream> = self.fields
            .into_iter()
            .map(|dependency| dependency.initializer_tokens())
            .collect();
        for singleton in singletons {
            initializer_tokens.push(singleton.initializer_tokens())
        }

        quote! {
            impl #name {
                pub fn construct(#(#argument_tokens),*) -> std::rc::Rc<Self> {
                    std::rc::Rc::new(Self {
                        #(#initializer_tokens),*
                    })
                }
            }
        }
    }

    pub fn scope_tokens(self, singletons: Vec<SingletonDependency>) -> TokenStream {
        let name = self.name;
        let mut initializer_tokens: Vec<TokenStream> = self.fields
            .into_iter()
            .map(|dependency| dependency.scope_tokens())
            .collect();
        for singleton in singletons {
            initializer_tokens.push(singleton.initializer_tokens())
        }

        quote! {
            impl curds_core_abstraction::dependency_injection::Scoped for #name {
                fn scope(&self) -> Self {
                    Self {
                        #(#initializer_tokens),*
                    }
                }
            }
        }
    }
}