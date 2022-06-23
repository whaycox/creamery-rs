use super::*;

pub struct WheyContext {
    tokens: TokenStream,
}

impl WheyContext {
    pub fn quote(self, context_type: Ident) -> TokenStream {
        let tokens = self.tokens;
        quote! {
            #[service_provider]
            #[generates_singleton(#context_type)]
            #tokens
        }
    }
}

impl Parse for WheyContext {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(WheyContext {
            tokens: input.parse()?,
        })
    }
}