use super::*;

#[derive(Clone)]
pub struct MessageDefinition {
    name: Ident,
    message_type: Type,
    base_name: Ident,
    routing: DispatchRouting,
}

impl MessageDefinition {
    pub fn parse(attribute: Attribute) -> Result<Self> {
        let name = attribute.path
            .get_ident()
            .unwrap()
            .clone();
        let formatted_name: String = name
            .clone()
            .to_string()
            .split("_")
            .into_iter()
            .map(|part| {
                let mut title_part = part.to_owned();
                if let Some(char) = title_part.get_mut(0..1) {
                    char.make_ascii_uppercase();
                }
                title_part
            })
            .collect();
        let routing: DispatchRouting = attribute.parse_args()?;

        Ok(Self {
            name: name,
            message_type: routing.message_type.clone(),
            base_name: Ident::new(&formatted_name, attribute.path.span()),
            routing: routing,
        })
    }

    pub fn context_type(&self) -> Type { self.routing.context_type.clone() }

    pub fn signature_tokens(self) -> TokenStream {
        let name = self.name;
        let message_type = self.message_type;
        let return_tokens = self.routing.return_tokens();
        quote! { fn #name(&self, message: #message_type) -> curds_core_abstraction::message_dispatch::Result<#return_tokens>; }
    }

    pub fn trait_tokens(self, visibility: &Visibility, parent_trait: &Ident) -> TokenStream { self.routing.trait_tokens(visibility, parent_trait, &self.base_name) }

    pub fn implementation_tokens(self) -> TokenStream {
        let name = self.name;
        let message_type = self.message_type;
        let return_tokens = self.routing.return_tokens();
        let routing = self.routing.quote(&self.base_name);
        quote! {
            #[allow(non_snake_case)]
            fn #name(&self, message: #message_type) -> curds_core_abstraction::message_dispatch::Result<#return_tokens> {
                #routing
            }
        }
    }
}