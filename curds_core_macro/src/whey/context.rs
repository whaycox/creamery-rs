use super::*;

pub struct WheyContext {
    item: ItemStruct,
}

impl WheyContext {
    pub fn quote(self, context_type: Ident) -> TokenStream {
        let item = self.item;
        quote! {
            #[service_provider]
            #[generates_singleton(#context_type)]
            #item
        }
    }
}

impl Parse for WheyContext {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(WheyContext {
            item: input.parse()?,
        })
    }
}