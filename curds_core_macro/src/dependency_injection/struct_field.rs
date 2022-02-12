use super::*;

#[derive(Clone)]
pub struct StructField {
    pub field: Field,
    pub default: bool,
}
impl StructField {
    pub fn eq(&self, name: &Ident) -> bool {
        self.field.ident.clone().unwrap().to_string() == name.to_string()
    }
    pub fn ty(&self) -> Type {
        self.field.ty.clone()
    }

    pub fn new(field: Field, default: bool) -> Self {
        Self {
            field: field,
            default: default,
        }
    }

    pub fn constraint_tokens(self) -> Option<TypeParamBound> {
        if !self.default {
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
            generic_arguments.args.push(GenericArgument::Type(self.field.ty));
            constraint_path.segments.push(PathSegment {
                ident: Ident::new("ServiceGenerator", Span::call_site()),
                arguments: PathArguments::AngleBracketed(generic_arguments),
            });
            Some(TypeParamBound::Trait(TraitBound {
                paren_token: None,
                modifier: TraitBoundModifier::None,
                lifetimes: None,
                path: constraint_path,
            }))
        }
        else {
            None
        }
    }

    pub fn generator_tokens(self) -> Option<TokenStream> {
        if !self.default {
            let dependency = self.field.ty;
            Some(quote! { curds_core_abstraction::dependency_injection::ServiceGenerator::<#dependency>::generate(provider) })
        }
        else {
            None
        }
    }

    pub fn argument_tokens(self) -> Option<TokenStream> {
        if !self.default {
            let name = self.field.ident.unwrap();
            let ty = self.field.ty;

            Some(quote! { #name: #ty })
        }
        else {
            None
        }
    }

    pub fn initializer_tokens(self) -> TokenStream {
        let name = self.field.ident.unwrap();
        if self.default {
            quote! { #name: std::default::Default::default() }
        }
        else {
            quote! { #name: #name }
        }
    }

    pub fn scope_tokens(self) -> TokenStream {
        let name = self.field.ident.unwrap();
        if self.default {
            quote! { #name: std::default::Default::default() }
        }
        else {
            quote! { #name: self.#name.clone() }
        }
    }
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.field.to_tokens(tokens)
    }
}