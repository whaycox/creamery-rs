use super::*;

pub struct EnumerationDescriptions {
    description: Option<LitStr>,
    variant_descriptions: HashMap<Ident, LitStr>,
}

impl EnumerationDescriptions {
    pub fn parse(item: &mut ItemEnum) -> Result<Self> {
        let mut description: Option<LitStr> = None;
        let attribute_length = item.attrs.len();
        if attribute_length > 0 {
            let mut attribute_index = 0;
            loop {
                let attribute = &item.attrs[attribute_index];
                if attribute.path.is_ident(DESCRIPTION_IDENTIFIER) {
                    description = Some(attribute.parse_args()?);
                    item.attrs.remove(attribute_index);
                    break;
                }
                attribute_index += 1;
                if attribute_index == attribute_length {
                    break;
                }
            }
        }

        let mut variant_descriptions: HashMap<Ident, LitStr> = HashMap::new();
        for variant in &mut item.variants {
            let attribute_length = variant.attrs.len();
            if attribute_length > 0 {
                let mut attribute_index = 0;
                loop {
                    let attribute = &variant.attrs[attribute_index];
                    if attribute.path.is_ident(DESCRIPTION_IDENTIFIER) {
                        variant_descriptions.insert(variant.ident.clone(), attribute.parse_args()?);
                        variant.attrs.remove(attribute_index);
                        break;
                    }
                    attribute_index += 1;
                    if attribute_index == attribute_length {
                        break;
                    }
                }
            }
        }

        Ok(Self {
            description,
            variant_descriptions,
        })
    }

    pub fn quote_descriptions(self) -> Vec<TokenStream> {
        let mut descriptions: Vec<TokenStream> = vec![];
        if let Some(enum_description) = self.description {
            descriptions.push(quote! { descriptions.push(#enum_description); });
        }
        if self.variant_descriptions.len() > 0 {
            descriptions.push(quote! { descriptions.push("Operations:"); });
            for (variant_name, variant_description) in self.variant_descriptions {
                let argument = format!("--{}", format_argument_name(&variant_name));
                descriptions.push(quote! { descriptions.push(#argument); });
                descriptions.push(quote! { descriptions.push(#variant_description); });
            }
        }

        if descriptions.len() > 0 {
            descriptions.insert(0, quote! { let mut descriptions: Vec<&'static str> = vec![]; });
            descriptions.push(quote! { Some(descriptions) });
        }
        descriptions
    }
}