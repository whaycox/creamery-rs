use std::collections::HashMap;

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

pub const CLONES_IDENTIFIER: &str = "clones";
pub const SCOPES_IDENTIFIER: &str = "scopes";
pub const SCOPES_SINGLETON_IDENTIFIER: &str = "scopes_singleton";
pub const GENERATES_IDENTIFIER: &str = "generates";
pub const GENERATES_SINGLETON_IDENTIFIER: &str = "generates_singleton";
pub const FORWARDS_IDENTIFIER: &str = "forwards";
pub const FORWARDS_SINGLETON_IDENTIFIER: &str = "forwards_singleton";

impl ServiceProduction {
    pub fn singleton_fields(library: Vec<Self>, struct_definition: &StructDefinition) -> Vec<SingletonDependency> {
        let mut singletons: HashMap<String, SingletonDependency> = HashMap::new();
        for production in library {
            match production {
                Self::GenerateSingleton(generated) => {
                    let singleton = generated.singleton
                        .ident()
                        .to_string();
                    if !singletons.contains_key(&singleton) {
                        singletons.insert(singleton, generated.to_singleton_dependency());
                    }
                },
                Self::ForwardSingleton(forwarded) => {
                    let singleton = forwarded.singleton
                        .ident()
                        .to_string();
                    if !singletons.contains_key(&singleton) {
                        singletons.insert(singleton, forwarded.to_singleton_dependency());
                    }
                },
                Self::ScopeSingleton(scoped) => {
                    let singleton = scoped.singleton
                        .ident()
                        .to_string();
                    if !singletons.contains_key(&singleton) {
                        singletons.insert(singleton, scoped.to_singleton_dependency(struct_definition));
                    }
                },
                _ => {}
            }
        }

        singletons
            .into_values()
            .collect()
    }

    pub fn parse_library(input: ParseStream) -> Result<Vec<Self>> {
        let mut singletons = SingletonCollection::new();
        let mut library: Vec<Self> = Vec::new();
        for attribute in Attribute::parse_outer(input)? {
            if attribute.path.is_ident(CLONES_IDENTIFIER) {
                library.push(Self::CloneTransient(attribute.parse_args::<ProviderDefinition>()?))
            }
            else if attribute.path.is_ident(SCOPES_IDENTIFIER) {
                library.push(Self::ScopeTransient(attribute.parse_args::<ProviderDefinition>()?))
            }
            else if attribute.path.is_ident(SCOPES_SINGLETON_IDENTIFIER) {
                let definition = attribute.parse_args::<ProviderDefinition>()?;                
                library.push(Self::ScopeSingleton(definition.register(&mut singletons)))
            }
            else if attribute.path.is_ident(GENERATES_IDENTIFIER) {
                library.push(Self::GenerateTransient(attribute.parse_args::<GeneratedDefinition>()?))
            }
            else if attribute.path.is_ident(GENERATES_SINGLETON_IDENTIFIER) {
                let definition = attribute.parse_args::<GeneratedDefinition>()?;
                library.push(Self::GenerateSingleton(definition.register(&mut singletons)))
            }
            else if attribute.path.is_ident(FORWARDS_IDENTIFIER) {
                library.push(Self::ForwardTransient(attribute.parse_args::<ForwardedDefinition>()?))
            }
            else if attribute.path.is_ident(FORWARDS_SINGLETON_IDENTIFIER) {
                let definition = attribute.parse_args::<ForwardedDefinition>()?;
                library.push(Self::ForwardSingleton(definition.register(&mut singletons)))
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