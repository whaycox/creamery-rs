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
        let crate_name = resolve_crate_name();
        let name = &item.ident;
        let initializer = parse_fields(quote! { #name }, &item.fields);
        let usage = field_usage(None, &item.fields);

        quote! {
            #item

            impl #crate_name::cli::CliArgumentParse for #name {
                fn parse(arguments: &mut Vec<String>) -> Result<Self, #crate_name::cli::CliArgumentParseError> {
                    #initializer
                }

                fn usage() -> String {
                    let mut usages: Vec<String> = vec![];
                    #usage
                    
                    usages.join(" ")
                 }
            }
        }
    }
}