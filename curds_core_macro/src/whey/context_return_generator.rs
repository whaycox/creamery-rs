use super::*;

pub struct WheyContextReturnGenerator {
    expected_mock: Path,
    method: Ident,
    generator: WheyExpectation,
    times: TokenStream,
}

impl Parse for WheyContextReturnGenerator {
    fn parse(input: ParseStream) -> Result<Self> {
        let expected_mock: Path = input.parse()?;
        input.parse::<Token![~]>()?;
        let method: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let generator: WheyExpectation = input.parse()?;
        input.parse::<Token![,]>()?;
        let times: TokenStream = input.parse()?;

        Ok(WheyContextReturnGenerator {
            expected_mock,
            method,
            generator,
            times,
        })
    }
}

impl WheyContextReturnGenerator {
    pub fn quote(self) -> TokenStream {
        let expected_mock = &self.expected_mock;
        let method = &self.method;
        let generator = self.generator;
        let times = self.times;

        quote! {
            curds_core_macro::mock_return!(self ~ #expected_mock ~ #method, #generator, #times);
        }
    }
}