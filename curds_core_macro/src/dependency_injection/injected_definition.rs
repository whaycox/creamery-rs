use super::*;

pub const DEFAULTED_IDENTIFIER: &str = "defaulted";

pub struct InjectedDefinition {
    item: ItemStruct,
    defaulted: HashMap<Ident, TokenStream>,
}
impl Parse for InjectedDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut item: ItemStruct = input.parse()?;
        let defaulted = Self::parse_defaulted(&mut item)?;

        Ok(InjectedDefinition {
            item,
            defaulted,
        })
    }
}

impl InjectedDefinition {
    fn parse_defaulted(item: &mut ItemStruct) -> Result<HashMap<Ident, TokenStream>> {
        let mut defaulted: HashMap<Ident, TokenStream> = HashMap::new();
        match &mut item.fields {
            Fields::Named(named_fields) => {
                for field in &mut named_fields.named {
                    let length = field.attrs.len();
                    if length > 0 {
                        let mut attribute_index = 0;
                        while attribute_index < length {
                            let attribute = &field.attrs[attribute_index];
                            if attribute.path.is_ident(DEFAULTED_IDENTIFIER) {
                                let ident = field.ident.clone().unwrap();
                                let mut default_value = quote! { std::default::Default::default() };
                                if !attribute.tokens.is_empty() {
                                    default_value = attribute.parse_args()?;
                                }
                                
                                defaulted.insert(ident, default_value);
                                field.attrs.remove(attribute_index);
                                break;
                            }

                            attribute_index = attribute_index + 1;
                        }
                    }
                }
            },
            _ => panic!("Only named fields are supported"),
        }

        Ok(defaulted)
    }

    pub fn quote(self) -> TokenStream {
        let item = &self.item;
        let injected = self.quote_injected();
        let construct = self.quote_construct();

        quote! {
            #item
            #injected
            #construct
        }
    }
    fn quote_injected(&self) -> TokenStream {
        let name = &self.item.ident;
        let generics_without_provider = &self.item.generics;
        let mut generics = generics_without_provider.clone();
        generics.params.push(GenericParam::Type(self.constraint_param()));
        
        let (impl_generics, _, where_clause) = generics.split_for_impl();
        let (_, type_generics, _) = generics_without_provider.split_for_impl();
        let generator_tokens = self.quote_generators();

        quote! {
            impl #impl_generics curds_core_abstraction::dependency_injection::Injected<TProvider> for #name #type_generics #where_clause {
                fn inject(provider: &TProvider) -> Self {
                    Self::construct(#generator_tokens)
                }
            }
        }
    }
    fn constraint_param(&self) -> TypeParam {
        let mut provider_generic = TypeParam::from(Ident::new("TProvider", Span::call_site()));
        for field in &self.item.fields {
            let name = &field.ident.clone().unwrap();
            if self.defaulted.contains_key(&name) {
                continue;
            }
            let mut constraint_path = Path::from(PathSegment {
                ident: Ident::new("curds_core_abstraction", Span::call_site()),
                arguments: PathArguments::None,
            });
            constraint_path.segments.push(PathSegment {
                ident: Ident::new("dependency_injection", Span::call_site()),
                arguments: PathArguments::None,
            });
            let mut generic_arguments = AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: syn::token::Lt { spans: [Span::call_site()] },
                args: Punctuated::new(),
                gt_token: syn::token::Gt { spans: [Span::call_site()] },
            };
            generic_arguments.args.push(GenericArgument::Type(field.ty.clone()));
            constraint_path.segments.push(PathSegment {
                ident: Ident::new("ServiceGenerator", Span::call_site()),
                arguments: PathArguments::AngleBracketed(generic_arguments),
            });            
            let bound = TypeParamBound::Trait(TraitBound {
                paren_token: None,
                modifier: TraitBoundModifier::None,
                lifetimes: None,
                path: constraint_path,
            });
            provider_generic.bounds.push(bound);
        }
        provider_generic
    }
    fn quote_generators(&self) -> TokenStream {
        let mut generator_tokens: Vec<TokenStream> = Vec::new();
        match &self.item.fields {
            Fields::Named(named) => {
                for field in &named.named {
                    let name = &field.ident.clone().unwrap();
                    if self.defaulted.contains_key(&name) {
                        continue;
                    }
                    let dependency = &field.ty;
                    generator_tokens.push(quote! { curds_core_abstraction::dependency_injection::ServiceGenerator::<#dependency>::generate(provider) })
                }
            },
            _ => panic!("Only named fields are supported"),
        }

        quote! {
            #(#generator_tokens),*
        }
    }
    fn quote_construct(&self) -> TokenStream {
        let name = &self.item.ident;
        let (impl_generics, type_generics, where_clause) = self.item.generics.split_for_impl();
        let arguments = self.quote_arguments();
        let initializers = self.quote_initializers();

        quote! {
            impl #impl_generics #name #type_generics #where_clause {
                pub fn construct(#arguments) -> Self {
                    Self {
                        #initializers
                    }
                }
            }
        }
    }
    fn quote_arguments(&self) -> TokenStream {
        let mut argument_tokens: Vec<TokenStream> = Vec::new();
        match &self.item.fields {
            Fields::Named(named) => {
                for field in &named.named {
                    let name = &field.ident.clone().unwrap();
                    if self.defaulted.contains_key(&name) {
                        continue;
                    }
                    let ty = &field.ty;
                    argument_tokens.push(quote! {
                        #name: #ty
                    })
                }
            },
            _ => panic!("Only named fields are supported"),
        }
        quote! {
            #(#argument_tokens),*
        }
    }
    fn quote_initializers(&self) -> TokenStream {
        let mut initializer_tokens: Vec<TokenStream> = Vec::new();
        match &self.item.fields {
            Fields::Named(named) => {
                for field in &named.named {
                    let name = &field.ident.clone().unwrap();
                    if self.defaulted.contains_key(&name) {
                        let default_value = &self.defaulted[name];
                        initializer_tokens.push(quote! { #name: #default_value })
                    }
                    else {
                        initializer_tokens.push(quote! { #name: #name })
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