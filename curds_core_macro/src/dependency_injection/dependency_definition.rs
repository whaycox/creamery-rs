use super::*;

#[derive(Clone)]
pub struct DependencyDefinition {
    pub ident: Ident,
    pub fields: Punctuated<Field, Token![,]>,
}