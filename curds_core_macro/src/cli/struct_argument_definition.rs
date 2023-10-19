use super::*;

pub struct CliArgumentStructDefinition {
    item: ItemStruct,
}

impl Parse for CliArgumentStructDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

impl CliArgumentStructDefinition {
    pub fn quote(self) -> TokenStream {
        let item = self.item;

        quote! {
            #item
        }
    }
}