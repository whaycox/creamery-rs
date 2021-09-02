use super::*;

#[derive(Clone)]
pub struct InjectedDependency {
    visibility: Visibility,
    name: Ident,
    ty: Type,
    pub default: bool,
}

impl InjectedDependency {
    pub fn parse(buffer: ParseBuffer, defaults: &HashSet<Ident>) -> Result<Vec<Self>> {
        let fields: Punctuated<Field, Token![,]> = buffer.parse_terminated(Field::parse_named)?;
        let mut dependencies: Vec<InjectedDependency> = Vec::new();
        for field in fields {
            dependencies.push(Self::parse_dependency(field, defaults)?)
        }

        Ok(dependencies)
    }
    fn parse_dependency(field: Field, defaults: &HashSet<Ident>) -> Result<Self> {
        Ok(Self {
            visibility: field.vis,
            name: field.ident.clone().unwrap(),
            ty: field.ty,
            default: defaults.contains(&field.ident.unwrap()),
        })
    }

    pub fn struct_tokens(self) -> TokenStream {
        let visibility = self.visibility;
        let name = self.name;
        let ty = self.ty;

        quote! {
            #visibility #name: #ty
        }
    }

    pub fn constraint_tokens(self) -> TokenStream {
        let required_dependency = self.ty;

        quote! { curds_core_abstraction::dependency_injection::ServiceGenerator<#required_dependency> }
    }

    pub fn generator_tokens(self) -> TokenStream {
        let required_dependency = self.ty;

        quote! { curds_core_abstraction::dependency_injection::ServiceGenerator::<#required_dependency>::generate(provider) }
    }

    pub fn argument_tokens(self) -> TokenStream {
        let name = self.name;
        let ty = self.ty;

        quote! { #name: #ty }
    }

    pub fn initializer_tokens(self) -> TokenStream {
        let name = self.name;
        if self.default {
            quote! { #name: std::default::Default::default() }
        }
        else {
            quote! { #name: #name }
        }
    }
}