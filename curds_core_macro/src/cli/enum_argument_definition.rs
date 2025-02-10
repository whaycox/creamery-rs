use super::*;

pub struct CliArgumentEnumerationDefinition {
    item: ItemEnum,
    descriptions: EnumerationDescriptions,
}

impl Parse for CliArgumentEnumerationDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut item: ItemEnum = input.parse()?;
        let descriptions = EnumerationDescriptions::parse(&mut item)?;

        Ok(Self {
            item,
            descriptions,
        })
    }
}

impl CliArgumentEnumerationDefinition {
    pub fn quote(self) -> TokenStream {
        let variants = self.quote_cli_argument_parse();
        let variant_usage = self.quote_usage();
        let item = self.item;
        let crate_name = resolve_crate_name();
        let name = &item.ident;
        let mut descriptions = self.descriptions.quote_descriptions();
        if descriptions.len() == 0 {
            descriptions = vec![quote! { None }];
        }

        quote! {
            #item

            impl #crate_name::cli::CliArgumentParse for #name {
                fn parse(arguments: &mut Vec<String>) -> Result<Self, #crate_name::cli::CliArgumentParseError> {
                    let key = arguments.pop().unwrap();
                    match key.as_str() {
                        #(#variants)*
                        _ => Err(#crate_name::cli::CliArgumentParseError::UnrecognizedKey(key)),
                    }
                }

                fn usage() -> String {
                    #variant_usage
                }

                fn description() -> Option<Vec<&'static str>> {
                    #(#descriptions)*
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
    fn quote_usage(&self) -> TokenStream {
        let mut variant_usages: Vec<TokenStream> = vec![];
        for variant in &self.item.variants {
            let variant_name = &variant.ident;
            let formatted_name = format_argument_name(variant_name);
            let argument = format!("--{}", formatted_name);

            variant_usages.push(field_usage(Some(argument), &variant.fields));
        }

        quote! {
            let mut usages: Vec<String> = vec![];
            #(#variant_usages)*
            
            usages.join(" ")
        }
    }
}