use super::*;

pub struct WheyContextDefaultReturn {
    expected_mock: Path,
    method: Ident,
    generator: WheyExpectation,
}

impl Parse for WheyContextDefaultReturn {
    fn parse(input: ParseStream) -> Result<Self> {
        let expected_mock: Path = input.parse()?;
        input.parse::<Token![~]>()?;
        let method: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let generator: WheyExpectation = input.parse()?;

        Ok(WheyContextDefaultReturn {
            expected_mock,
            method,
            generator,
        })
    }
}

impl WheyContextDefaultReturn {
    pub fn quote(self) -> TokenStream {
        let expected_mock = &self.expected_mock;
        let method = &self.method;
        let generator = self.generator;

        quote! {
            curds_core_macro::mock_default_return!(self ~ #expected_mock ~ #method, #generator);
        }
    }
}