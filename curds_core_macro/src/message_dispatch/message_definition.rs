use super::*;

pub struct MessageDefinition {
    visibility: bool,
    message_type: Ident,
    context_type: Ident,
}

impl Parse for MessageDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let visibility: Option<Token![pub]> = input.parse()?;
        let message_type: Ident = input.parse()?;
        input.parse::<Token![<-]>()?;
        let context_type: Ident = input.parse()?;
        if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            let return_type: Ident = input.parse()?;
        }
        else if input.peek(Token![&]) {
            input.parse::<Token![&]>()?;
            let pipeline_content;
            braced!(pipeline_content in input);
            let pipeline: PipelineDefinition = input.parse()?;
        }
        else if input.peek(Token![|]) {
            input.parse::<Token![|]>()?;
            let chain_content;
            braced!(chain_content in input);
            let chain: ChainDefinition = input.parse()?;
        }

        Ok(Self {
            visibility: visibility.is_some(),
            message_type: message_type,
            context_type: context_type,
        })
    }
}