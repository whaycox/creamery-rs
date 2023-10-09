use super::*;

pub struct MessageDefinition {
    message_name: Ident,
    base_name: Ident,
    routing: DispatchRouting,
}

impl MessageDefinition {
    pub fn new(message_name: &Ident, span: Span, routing: DispatchRouting) -> Self {
        let formatted_name: String = message_name
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

        Self {
            message_name: message_name.clone(),
            base_name: Ident::new(&formatted_name, span),
            routing,
        }
    }

    pub fn context_type(&self) -> Type { self.routing.context_type() }

    pub fn signature(&self, message_trait: &MessageTraitDefinition) -> TokenStream {
        let name = &self.message_name;
        let receiver_token = if self.routing.mutable() {
            quote! { &mut self }
        }
        else {
            quote! { &self }
        };
        let message = self.routing.message_type();
        let message_return = self.routing.return_type(&message_trait.error_type);

        quote! {
            fn #name(#receiver_token, message: #message) -> #message_return;
        }
    }

    pub fn trait_tokens(&self, visibility: &Visibility, message_trait: &MessageTraitDefinition) -> TokenStream {
        self.routing.trait_tokens(visibility, message_trait, &self.base_name)
    }

    pub fn implementation_tokens(&self, message_trait: &MessageTraitDefinition) -> TokenStream {
        let name = &self.message_name;
        let receiver_token = if self.routing.mutable() {
            quote! { &mut self }
        }
        else {
            quote! { &self }
        };
        let message = self.routing.message_type();
        let message_return = self.routing.return_type(&message_trait.error_type);
        let routing_implementation = self.routing.implementation_tokens(&self.base_name);

        quote! {
            #[allow(non_snake_case)]
            fn #name(#receiver_token, message: #message) -> #message_return {
                #routing_implementation
            }
        }
    }
}