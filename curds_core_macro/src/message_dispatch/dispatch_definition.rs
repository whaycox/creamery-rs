use super::*;

const MESSAGE_IDENTIFIER: &str = "message";
const REQUEST_IDENTIFIER: &str = "request";
const CHAIN_IDENTIFIER: &str = "chain";

pub struct DispatchDefinition {
    messages: Vec<MessageDefinition>,
    defaults: DispatchDefaults,
    provider_definition: ServiceProviderDefinition,
}

impl Parse for DispatchDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut messages: Vec<MessageDefinition> = Vec::new();
        let mut message_template: SerialTemplate = SerialTemplate::message();
        let mut request_template: SerialTemplate = SerialTemplate::request();
        let mut chain_template: ParallelTemplate = Default::default();
        let attributes = Attribute::parse_outer(&input.fork())?;
        let mut provider_definition: ServiceProviderDefinition = input.parse()?;
        let mut contexts: HashSet<Type> = HashSet::new();
        for attribute in attributes {
            if attribute.path.is_ident(MESSAGE_IDENTIFIER) {
                message_template = attribute.parse_args_with(SerialTemplate::parse_message)?;
            }
            else if attribute.path.is_ident(REQUEST_IDENTIFIER) {
                request_template = attribute.parse_args_with(SerialTemplate::parse_request)?;
            }
            else if attribute.path.is_ident(CHAIN_IDENTIFIER) {
                chain_template = attribute.parse_args_with(ParallelTemplate::parse)?;
            }
            else if !attribute.path.is_ident(CLONES_IDENTIFIER) &&
                !attribute.path.is_ident(SCOPES_IDENTIFIER) &&
                !attribute.path.is_ident(GENERATES_IDENTIFIER) &&
                !attribute.path.is_ident(GENERATES_SINGLETON_IDENTIFIER) &&
                !attribute.path.is_ident(FORWARDS_IDENTIFIER) &&
                !attribute.path.is_ident(FORWARDS_SINGLETON_IDENTIFIER) &&
                !attribute.path.is_ident(DEFAULTED_IDENTIFIER) {
                let message = MessageDefinition::parse(attribute)?;
                contexts.insert(message.context_type());
                messages.push(message);
            }
        }
        // for context in contexts {
        //     provider_definition.generates(context)
        // }

        Ok(Self {
            messages: messages,
            defaults: DispatchDefaults::new(
                message_template,
                request_template,
                chain_template,
            ),
            provider_definition: provider_definition,
        })
    }
}

impl DispatchDefinition {
    pub fn quote(self, message_trait: Ident) -> TokenStream {
        // let visibility = self.provider_definition.visibility();
        // let defaults = self.defaults;
        // let expanded_messages: Vec<MessageDefinition> = self.messages
        //     .into_iter()
        //     .map(|message| message.expand(&defaults))
        //     .collect();

        // let message_signatures: Vec<TokenStream> = expanded_messages
        //     .clone()
        //     .into_iter()
        //     .map(|message| message.signature_tokens())
        //     .collect();
        // let provider_definition = self.provider_definition
        //     .clone()
        //     .quote();
        // let message_traits: Vec<TokenStream> = expanded_messages
        //     .clone()
        //     .into_iter()
        //     .map(|message| message.trait_tokens(&visibility, &message_trait))
        //     .collect();
        // let provider_name = self.provider_definition.name();
        // let (impl_generics, type_generics, where_clause) = self.provider_definition.definition.generics.split_for_impl();
        // let message_implementations: Vec<TokenStream> = expanded_messages
        //     .into_iter()
        //     .map(|message| message.implementation_tokens())
        //     .collect();

        quote! {
            // #visibility trait #message_trait {
            //     #(#message_signatures)*
            // }
            // #provider_definition

            // #(#message_traits)*

            // impl #impl_generics #message_trait for #provider_name #type_generics #where_clause {
            //     #(#message_implementations)*
            // }
        }
    }
}