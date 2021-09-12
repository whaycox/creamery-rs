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

    pub fn constraint_tokens(self) -> Option<TokenStream> {
        if !self.default {
            let dependency = self.field.ty;
            Some(quote! { curds_core_abstraction::dependency_injection::ServiceGenerator<#dependency> })
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