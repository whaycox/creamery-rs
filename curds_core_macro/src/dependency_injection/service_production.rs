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

const CLONES_IDENTIFIER: &str = "clones";
const SCOPES_IDENTIFIER: &str = "scopes";
const SCOPES_SINGLETON_IDENTIFIER: &str = "scopes_singleton";
const GENERATES_IDENTIFIER: &str = "generates";
const GENERATES_SINGLETON_IDENTIFIER: &str = "generates_singleton";
const FORWARDS_IDENTIFIER: &str = "forwards";
const FORWARDS_SINGLETON_IDENTIFIER: &str = "forwards_singleton";

impl ServiceProduction {
    pub fn singleton_fields(library: Vec<Self>, struct_definition: &StructDefinition) -> Vec<SingletonDependency> {
        let mut singletons: HashMap<String, SingletonDependency> = HashMap::new();
        for production in library {
            match production {
                Self::GenerateSingleton(generated) => {
                    let singleton_type = generated.singleton_type();
                    if !singletons.contains_key(&singleton_type) {
                        singletons.insert(singleton_type, generated.to_singleton_dependency());
                    }
                },
                Self::ForwardSingleton(forwarded) => {
                    let singleton_type = forwarded.singleton_type();
                    if !singletons.contains_key(&singleton_type) {
                        singletons.insert(singleton_type, forwarded.to_singleton_dependency());
                    }
                },
                Self::ScopeSingleton(scoped) => {
                    let singleton_type = scoped.provider_name();
                    if !singletons.contains_key(&singleton_type) {
                        singletons.insert(singleton_type, scoped.to_singleton_dependency(struct_definition));
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
        let mut singletons: HashMap<String, SingletonIdentifier> = HashMap::new();
        let mut library: Vec<Self> = Vec::new();
        for attribute in Attribute::parse_outer(input)? {
            if attribute.path.is_ident(CLONES_IDENTIFIER) {
                library.push(Self::CloneTransient(attribute.parse_args::<ProviderDefinition>()?))
            }
            else if attribute.path.is_ident(SCOPES_IDENTIFIER) {
                library.push(Self::ScopeTransient(attribute.parse_args::<ProviderDefinition>()?))
            }
            else if attribute.path.is_ident(SCOPES_SINGLETON_IDENTIFIER) {
                let mut definition = attribute.parse_args::<ProviderDefinition>()?;
                let singleton_type = definition.provider_name();
                match singletons.get(&singleton_type) {
                    Some(singleton) => {
                        definition = definition.set_singleton_identifier(singleton);
                    },
                    None => {
                        singletons.insert(singleton_type, definition.singleton.clone());
                    }
                }
                
                library.push(Self::ScopeSingleton(definition))
            }
            else if attribute.path.is_ident(GENERATES_IDENTIFIER) {
                library.push(Self::GenerateTransient(attribute.parse_args::<GeneratedDefinition>()?))
            }
            else if attribute.path.is_ident(GENERATES_SINGLETON_IDENTIFIER) {
                let mut definition = attribute.parse_args::<GeneratedDefinition>()?;
                let singleton_type = definition.singleton_type();
                match singletons.get(&singleton_type) {
                    Some(singleton) => {
                        definition = definition.set_singleton_identifier(singleton);
                    },
                    None => {
                        singletons.insert(singleton_type, definition.singleton.clone());
                    }
                }

                library.push(Self::GenerateSingleton(definition))
            }
            else if attribute.path.is_ident(FORWARDS_IDENTIFIER) {
                library.push(Self::ForwardTransient(attribute.parse_args::<ForwardedDefinition>()?))
            }
            else if attribute.path.is_ident(FORWARDS_SINGLETON_IDENTIFIER) {
                let mut definition = attribute.parse_args::<ForwardedDefinition>()?;
                let singleton_type = definition.singleton_type();
                match singletons.get(&singleton_type) {
                    Some(singleton) => {
                        definition = definition.set_singleton_identifier(singleton);
                    },
                    None => {
                        singletons.insert(singleton_type, definition.singleton.clone());
                    }
                }

                library.push(Self::ForwardSingleton(definition))
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