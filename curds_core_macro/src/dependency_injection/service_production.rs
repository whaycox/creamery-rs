use super::*;

#[derive(Clone)]
pub enum ServiceProduction {
    CloneTransient(ProviderDefinition),
    ScopeTransient(ProviderDefinition),
    ScopeSingleton(ProviderDefinition),
    GenerateTransient(GeneratedDefinition),
    GenerateSingleton(GeneratedDefinition),
    ForwardTransient(ForwardedDefinition),
    ForwardSingleton(ForwardedDefinition),
}

const CLONES_IDENTIFIER: &str = "clones";
const SCOPES_IDENTIFIER: &str = "scopes";
const SCOPES_SINGLETON_IDENTIFIER: &str = "scopes_singleton";
const GENERATES_IDENTIFIER: &str = "generates";
const GENERATES_SINGLETON_IDENTIFIER: &str = "generates_singleton";
const FORWARDS_IDENTIFIER: &str = "forwards";
const FORWARDS_SINGLETON_IDENTIFIER: &str = "forwards_singleton";

impl ServiceProduction {
    pub fn is_singleton(&self) -> bool {
        match self {
            Self::ForwardSingleton(_) |
            Self::GenerateSingleton(_) |
            Self::ScopeSingleton(_) => true,
            _ => false,
        }
    }

    pub fn singleton_dependency(self, definition: &StructDefinition) -> InjectedDependency {
        match self {
            Self::ForwardSingleton(forwarded) => forwarded.singleton_dependency(),
            Self::GenerateSingleton(generated) => generated.singleton_dependency(),
            Self::ScopeSingleton(scoped) => scoped.singleton_dependency(definition),
            _ => panic!(),
        }
    }

    pub fn parse(input: ParseStream) -> Result<Vec<Self>> {
        let mut library: Vec<Self> = Vec::new();
        for attribute in Attribute::parse_outer(input)? {
            if attribute.path.is_ident(CLONES_IDENTIFIER) {
                library.push(Self::CloneTransient(attribute.parse_args::<ProviderDefinition>()?))
            }
            else if attribute.path.is_ident(SCOPES_IDENTIFIER) {
                library.push(Self::ScopeTransient(attribute.parse_args::<ProviderDefinition>()?))
            }
            else if attribute.path.is_ident(SCOPES_SINGLETON_IDENTIFIER) {
                library.push(Self::ScopeSingleton(attribute.parse_args::<ProviderDefinition>()?))
            }
            else if attribute.path.is_ident(GENERATES_IDENTIFIER) {
                library.push(Self::GenerateTransient(attribute.parse_args::<GeneratedDefinition>()?))
            }
            else if attribute.path.is_ident(GENERATES_SINGLETON_IDENTIFIER) {
                library.push(Self::GenerateSingleton(attribute.parse_args::<GeneratedDefinition>()?))
            }
            else if attribute.path.is_ident(FORWARDS_IDENTIFIER) {
                library.push(Self::ForwardTransient(attribute.parse_args::<ForwardedDefinition>()?))
            }
            else if attribute.path.is_ident(FORWARDS_SINGLETON_IDENTIFIER) {
                library.push(Self::ForwardSingleton(attribute.parse_args::<ForwardedDefinition>()?))
            }
        }

        Ok(library)
    }

    pub fn quote(self, definition: &StructDefinition) -> TokenStream {
        match self {
            Self::CloneTransient(clone) => clone.clone_tokens(definition),
            Self::ScopeTransient(scope) => scope.scope_transient(definition),
            Self::ScopeSingleton(scope) => scope.scope_singleton(definition),
            Self::GenerateTransient(generate) => generate.transient(definition),
            Self::GenerateSingleton(generate) => generate.singleton(definition),
            Self::ForwardTransient(forward) => forward.transient(definition),
            Self::ForwardSingleton(forward) => forward.singleton(definition),
        }
    }
}