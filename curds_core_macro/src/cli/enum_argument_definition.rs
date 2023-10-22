use super::*;

pub struct CliArgumentEnumerationDefinition {
    item: ItemEnum,
}

impl Parse for CliArgumentEnumerationDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

impl CliArgumentEnumerationDefinition {
    pub fn quote(self) -> TokenStream {
        let variants = self.quote_cli_argument_parse();
        let item = self.item;
        let name = &item.ident;

        quote! {
            #item

            impl curds_core_abstraction::cli::CliArgumentParse for #name {
                fn parse(arguments: &mut Vec<String>) -> Self {
                    let key = arguments.pop().unwrap(); 
                    match key.as_str() {
                        #(#variants)*
                        _ => panic!("value \"{}\" not recognized as an operation key", key),
                    }
                }
            }
        }
    }

    fn quote_cli_argument_parse(&self) -> Vec<TokenStream> {
        let mut variants: Vec<TokenStream> = vec![];
        for variant in &self.item.variants {
            let variant_name = &variant.ident;
            let formatted_name = format_argument_name(variant_name);
            let argument = format!("--{}", formatted_name);
            let initializer = parse_fields(quote! { Self::#variant_name }, &variant.fields);

            variants.push(quote! { #argument => { #initializer }, });
        }

        variants
    }
}