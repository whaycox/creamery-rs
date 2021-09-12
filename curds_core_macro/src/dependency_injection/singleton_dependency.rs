use super::*;

#[derive(Clone)]
pub struct SingletonDependency {
    ident: SingletonIdentifier,
    ty: TokenStream,
}

impl SingletonDependency {
    pub fn new(ident: SingletonIdentifier, ty: TokenStream) -> Self {
        Self {
            ident: ident,
            ty: ty,
        }
    }

    pub fn field_tokens(self) -> TokenStream {
        let ident = self.ident.ident();
        let ty = self.ty;
        quote! { #ident: std::cell::RefCell<std::option::Option<#ty>> }
    }

    pub fn initializer_tokens(self) -> TokenStream {
        let ident = self.ident.ident();
        quote! { #ident: std::default::Default::default() }
    }
}