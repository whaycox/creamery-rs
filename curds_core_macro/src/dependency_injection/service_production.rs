use super::*;

pub enum ServiceProduction {
    CloneProvider(GeneratedDefinition),
    MapClonedProvider(MappedDefinition),
    ScopeProvider(GeneratedDefinition),
    MapScopedProvider(MappedDefinition),
    TransientGenerate(GeneratedDefinition),
    TransientMap(MappedDefinition),
    SingletonGenerate(GeneratedDefinition),
    SingletonMap(MappedDefinition),
    ForwardTransient(ForwardedDefinition),
    ForwardSingleton(ForwardedDefinition),
}

const CLONES_IDENTIFIER: &str = "clones";
const MAPS_CLONE_IDENTIFIER: &str = "maps_cloned";
const SCOPES_IDENTIFIER: &str = "scopes";
const MAPS_SCOPE_IDENTIFIER: &str = "maps_scoped";
const GENERATES_IDENTIFIER: &str = "generates";
const MAPS_IDENTIFIER: &str = "maps";
const GENERATES_SINGLETON_IDENTIFIER: &str = "generates_singleton";
const MAPS_SINGLETON_IDENTIFIER: &str = "maps_singleton";
const FORWARDS_IDENTIFIER: &str = "forwards";
const FORWARDS_SINGLETON_IDENTIFIER: &str = "forwards_singleton";

impl ServiceProduction {
    pub fn parse(input: ParseStream) -> Result<Vec<Self>> {
        let mut library: Vec<Self> = Vec::new();
        for attribute in Attribute::parse_outer(input)? {
            if attribute.path.is_ident(CLONES_IDENTIFIER) {
                library.push(Self::CloneProvider(attribute.parse_args::<GeneratedDefinition>()?))
            }
            else if attribute.path.is_ident(MAPS_CLONE_IDENTIFIER) {
                library.push(Self::MapClonedProvider(attribute.parse_args::<MappedDefinition>()?))
            }
            else if attribute.path.is_ident(SCOPES_IDENTIFIER) {
                library.push(Self::ScopeProvider(attribute.parse_args::<GeneratedDefinition>()?))
            }
            else if attribute.path.is_ident(MAPS_SCOPE_IDENTIFIER) {
                library.push(Self::MapScopedProvider(attribute.parse_args::<MappedDefinition>()?))
            }
            else if attribute.path.is_ident(GENERATES_IDENTIFIER) {
                library.push(Self::TransientGenerate(attribute.parse_args::<GeneratedDefinition>()?))
            }
            else if attribute.path.is_ident(MAPS_IDENTIFIER) {
                library.push(Self::TransientMap(attribute.parse_args::<MappedDefinition>()?))
            }
            else if attribute.path.is_ident(GENERATES_SINGLETON_IDENTIFIER) {
                library.push(Self::SingletonGenerate(attribute.parse_args::<GeneratedDefinition>()?))
            }
            else if attribute.path.is_ident(MAPS_SINGLETON_IDENTIFIER) {
                library.push(Self::SingletonMap(attribute.parse_args::<MappedDefinition>()?))
            }
            else if attribute.path.is_ident(FORWARDS_IDENTIFIER) {
                library.push(Self::ForwardTransient(attribute.parse_args_with(ForwardedDefinition::parse_transient)?))
            }
            else if attribute.path.is_ident(FORWARDS_SINGLETON_IDENTIFIER) {
                library.push(Self::ForwardSingleton(attribute.parse_args_with(ForwardedDefinition::parse_singleton)?))
            }
        }

        Ok(library)
    }

    pub fn quote(self, definition: &StructDefinition) -> TokenStream {
        match self {
            Self::TransientGenerate(generate) => generate.transient(definition),
            Self::TransientMap(map) => map.transient(definition),
            Self::SingletonGenerate(generate) => generate.singleton(definition),
            Self::SingletonMap(map) => map.singleton(definition),
            Self::ForwardTransient(forward) => forward.transient(definition),
            Self::ForwardSingleton(forward) => forward.singleton(definition),
            _ => panic!(),
        }
    }
}