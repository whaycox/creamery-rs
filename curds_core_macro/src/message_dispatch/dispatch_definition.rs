use super::*;

pub struct DispatchDefinition {
    messages: Vec<MessageDefinition>,
    struct_definition: StructDefinition,
}

impl Parse for DispatchDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = Attribute::parse_outer(&input.fork())?;
        for attribute in attributes {
            let name = attribute.path.get_ident().unwrap();
            let message: MessageDefinition = attribute.parse_args()?;
        }
        let struct_definition: StructDefinition = input.parse()?;

        Ok(Self {
            messages: Vec::new(),
            struct_definition: struct_definition,
        })
    }
}

impl DispatchDefinition {
    pub fn quote(self, message_trait: Ident) -> TokenStream {
        quote! {
            pub trait TestMessages {
                fn foo_message(&self, message: FooMessage) -> Result<()>;
            }
        }
    }
}