use super::*;

pub enum ServiceProduction {
    ForwardProvider(ForwardProviderDefinition),
    TransientMap(TransientMapDefinition),
}

const FORWARDS_IDENTIFIER: &str = "forwards";
const MAPS_IDENTIFIER: &str = "maps";

impl ServiceProduction {
    pub fn parse(input: ParseStream) -> Result<Vec<ServiceProduction>> {
        let mut library: Vec<ServiceProduction> = Vec::new();
        for attribute in Attribute::parse_outer(input)? {
            if attribute.path.is_ident(FORWARDS_IDENTIFIER) {
                library.push(ServiceProduction::ForwardProvider(attribute.parse_args::<ForwardProviderDefinition>()?))
            }
            else if attribute.path.is_ident(MAPS_IDENTIFIER) {
                library.push(ServiceProduction::TransientMap(attribute.parse_args::<TransientMapDefinition>()?))
            }
        }

        Ok(library)
    }

    pub fn quote(self, dependency_definition: &DependencyDefinition) -> TokenStream {
        match self {
            ServiceProduction::ForwardProvider(forward_provider) => forward_provider.quote(dependency_definition),
            ServiceProduction::TransientMap(transient_map) => transient_map.quote(dependency_definition),
        }
    }
}