use super::*;

pub enum ServiceProduction {
    ForwardProvider(ForwardProviderDefinition),
    CloneProvider(CloneProviderDefinition),
    TransientGenerate(TransientGenerateDefinition),
    TransientMap(TransientMapDefinition),
    SingletonGenerate(SingletonGenerateDefinition),
}

const FORWARDS_IDENTIFIER: &str = "forwards";
const CLONES_IDENTIFIER: &str = "clones";
const GENERATES_IDENTIFIER: &str = "generates";
const MAPS_IDENTIFIER: &str = "maps";
const GENERATES_SINGLETON_IDENTIFIER: &str = "generates_singleton";

impl ServiceProduction {
    pub fn parse(input: ParseStream) -> Result<Vec<Self>> {
        let mut library: Vec<Self> = Vec::new();
        for attribute in Attribute::parse_outer(input)? {
            if attribute.path.is_ident(FORWARDS_IDENTIFIER) {
                library.push(Self::ForwardProvider(attribute.parse_args::<ForwardProviderDefinition>()?))
            }
            else if attribute.path.is_ident(CLONES_IDENTIFIER) {
                library.push(Self::CloneProvider(attribute.parse_args::<CloneProviderDefinition>()?))
            }
            else if attribute.path.is_ident(GENERATES_IDENTIFIER) {
                library.push(Self::TransientGenerate(attribute.parse_args::<TransientGenerateDefinition>()?))
            }
            else if attribute.path.is_ident(MAPS_IDENTIFIER) {
                library.push(Self::TransientMap(attribute.parse_args::<TransientMapDefinition>()?))
            }
            else if attribute.path.is_ident(GENERATES_SINGLETON_IDENTIFIER) {
                library.push(Self::SingletonGenerate(attribute.parse_args::<SingletonGenerateDefinition>()?))
            }
        }

        Ok(library)
    }

    pub fn quote(self, dependency_definition: &DependencyDefinition) -> TokenStream {
        match self {
            Self::ForwardProvider(forward_provider) => forward_provider.quote(dependency_definition),
            Self::CloneProvider(clone_provider) => clone_provider.quote(dependency_definition),
            Self::TransientGenerate(transient_generate) => transient_generate.quote(dependency_definition),
            Self::TransientMap(transient_map) => transient_map.quote(dependency_definition),
            Self::SingletonGenerate(singleton_generate) => singleton_generate.quote(dependency_definition),
        }
    }
}