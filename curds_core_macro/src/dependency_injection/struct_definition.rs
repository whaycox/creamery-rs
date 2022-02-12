use super::*;

pub const DEFAULTED_IDENTIFIER: &str = "defaulted";

#[derive(Clone)]
pub struct StructDefinition {
    pub visibility: Visibility,
    pub name: Ident,
    generics: Generics,
    fields: Vec<StructField>,
}
impl Parse for StructDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        Attribute::parse_outer(input)?;
        let visibility: Visibility = input.parse()?;
        input.parse::<Token![struct]>()?;
        let name: Ident = input.parse()?;
        let generics: Generics = input.parse()?;
        let content;
        braced!(content in input);
        let parsed_fields: Punctuated<Field, Token![,]> = content.parse_terminated(Field::parse_named)?;
        let mut fields: Vec<StructField> = Vec::new();
        for mut field in parsed_fields {
            let mut default = false;
            for i in 0..field.attrs.len() {
                if field.attrs[i].path.is_ident(DEFAULTED_IDENTIFIER) {
                    default = true;
                    field.attrs.remove(i);
                    break;
                }
            }
            fields.push(StructField::new(field, default));
        }

        Ok(Self {
            visibility: visibility,
            name: name,
            generics: generics,
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
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();
        let mut struct_fields: Vec<TokenStream> = self.fields
            .into_iter()
            .map(|field| field.to_token_stream())
            .collect();
        for singleton in singletons {
            struct_fields.push(singleton.field_tokens())
        }

        quote! {
            #visibility struct #name #type_generics #where_clause {
                #(#struct_fields),*
            }
        }
    }
    fn injected_tokens(self) -> TokenStream {
        let name = self.name;
        let mut provider_generic = TypeParam::from(Ident::new("TProvider", Span::call_site()));
        for field in self.fields.clone() {
            let bound = field.constraint_tokens();
            if bound.is_some() {
                provider_generic.bounds.push(bound.unwrap())
            }
        }
        let mut generics = self.generics;
        let generics_without_provider = generics.clone();
        generics.params.push(GenericParam::Type(provider_generic));
        let generator_tokens = self.fields
            .into_iter()
            .filter_map(|dependency| dependency.generator_tokens());
        let (impl_generics, _, where_clause) = generics.split_for_impl();
        let (_, type_generics, _) = generics_without_provider.split_for_impl();
        
        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::Injected<TProvider> for #name #type_generics #where_clause {
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
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        quote! {
            impl #impl_generics #name #type_generics #where_clause {
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
        let (impl_generics, type_generics, where_clause) = self.generics.split_for_impl();

        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::Scoped for #name #type_generics #where_clause {
                fn scope(&self) -> Self {
                    Self {
                        #(#initializer_tokens),*
                    }
                }
            }
        }
    }
}