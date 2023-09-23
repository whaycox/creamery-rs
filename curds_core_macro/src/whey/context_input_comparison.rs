use super::*;

pub struct WheyContextInputComparison {
    expected_mock: Path,
    method: Ident,
    comparison: WheyExpectation,
    times: TokenStream,
}

impl Parse for WheyContextInputComparison {
    fn parse(input: ParseStream) -> Result<Self> {
        let expected_mock: Path = input.parse()?;
        input.parse::<Token![~]>()?;
        let method: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let comparison: WheyExpectation = input.parse()?;
        input.parse::<Token![,]>()?;
        let times: TokenStream = input.parse()?;

        Ok(WheyContextInputComparison {
            expected_mock,
            method,
            comparison,
            times,
        })
    }
}

impl WheyContextInputComparison {
    pub fn quote(self) -> TokenStream {
        let expected_mock = &self.expected_mock;
        let method = &self.method;
        let comparison = self.comparison;
        let times = self.times;

        quote! {
            curds_core_macro::mock_input!(self ~ #expected_mock ~ #method, #comparison, #times);
        }
    }
}