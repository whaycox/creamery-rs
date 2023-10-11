use super::*;

pub const GENERATES_IDENTIFIER: &str = "generates";
pub const GENERATES_SINGLETON_IDENTIFIER: &str = "generates_singleton";
pub const CLONES_SELF_IDENTIFIER: &str = "clones_self";
pub const CLONES_IDENTIFIER: &str = "clones";
pub const SCOPES_SELF_IDENTIFIER: &str = "scopes_self";
pub const SCOPES_IDENTIFIER: &str = "scopes";
pub const FORWARDS_IDENTIFIER: &str = "forwards";
pub const FORWARDS_SINGLETON_IDENTIFIER: &str = "forwards_singleton";

pub enum ServiceProduction {
    GenerateTransient(GeneratedDefinition),
    GenerateSingleton(GeneratedDefinition),
    CloneSelf(),
    Clone(ProviderDefinition),
    ForwardTransient(ForwardedDefinition),
    ForwardSingleton(ForwardedDefinition),
    ScopeSelf(),
    ScopeTransient(ProviderDefinition),
}

impl ServiceProduction {
    pub fn quote(&self, provider: &ServiceProviderDefinition) -> TokenStream {
        match self {
            ServiceProduction::GenerateTransient(generates) => generates.quote_transient(provider),
            ServiceProduction::GenerateSingleton(generates) => generates.quote_singleton(provider),
            ServiceProduction::CloneSelf() => Self::quote_clone_self(provider),
            ServiceProduction::Clone(clones) => clones.quote_clone(provider),
            ServiceProduction::ForwardTransient(forwards) => forwards.quote_transient(provider),
            ServiceProduction::ForwardSingleton(forwards) => forwards.quote_singleton(provider),
            ServiceProduction::ScopeSelf() => Self::quote_scope_self(provider),
            ServiceProduction::ScopeTransient(scopes) => scopes.quote_scope(provider),
        }
    }
    fn quote_clone_self(provider: &ServiceProviderDefinition) -> TokenStream {
        let name = provider.name();
        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<Self> for #name {
                fn generate(&self) -> Self {
                    std::clone::Clone::clone(self)
                }
            }
        }
    }
    fn quote_scope_self(provider: &ServiceProviderDefinition) -> TokenStream {
        let name = provider.name();
        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<Self> for #name {
                fn generate(&self) -> Self {
                    curds_core_abstraction::dependency_injection::Scoped::scope(self)
                }
            }
        }
    }
}