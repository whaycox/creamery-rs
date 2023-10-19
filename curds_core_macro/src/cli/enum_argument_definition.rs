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

        quote! {
            #item

            impl CliArgumentParse for TestOperations {
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
            let argument_data = match &variant.fields {
                Fields::Unit => quote! {},
                Fields::Unnamed(fields) => {
                    let mut unnamed_fields: Vec<TokenStream> = vec![];
                    for field in &fields.unnamed {
                        let ty = &field.ty;
                        unnamed_fields.push(quote! { <#ty as curds_core_abstraction::cli::CliArgumentParse>::parse(arguments) })
                    }

                    quote! { (#(#unnamed_fields),*) }
                },
                Fields::Named(fields) => quote! {},
            };
            println!("VARIANT: {:?}", variant);

            variants.push(quote! {
                #argument => Self::#variant_name #argument_data,
            })
        }

        variants
    }
}