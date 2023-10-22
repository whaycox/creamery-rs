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
        let name = &item.ident;
        let initializer = parse_fields(quote! { #name }, &item.fields);

        quote! {
            #item

            impl curds_core_abstraction::cli::CliArgumentParse for #name {
                fn parse(arguments: &mut Vec<String>) -> Self {
                    #initializer
                }
            }
        }
    }
}