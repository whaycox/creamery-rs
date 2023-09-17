use super::*;

pub struct WheyInputComparison {
    context: Ident,
    expected_mock: Path,
    method: Ident,
    comparison: ExprClosure,
    times: TokenStream,
}

impl Parse for WheyInputComparison {
    fn parse(input: ParseStream) -> Result<Self> {
        let context: Ident = input.parse()?;
        input.parse::<Token![~]>()?;
        let expected_mock: Path = input.parse()?;
        input.parse::<Token![~]>()?;
        let method: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let comparison: ExprClosure = input.parse()?;
        input.parse::<Token![,]>()?;
        let times: TokenStream = input.parse()?;

        Ok(WheyInputComparison {
            context,
            expected_mock,
            method,
            comparison,
            times,
        })
    }
}

impl WheyInputComparison {
    pub fn quote(self) -> TokenStream {
        let context = self.context;
        let expected_mock = MockedTraitDefinition::generate_core_name(&self.expected_mock);
        let method = WheyMockCore::store_expected_input(&self.method);
        let comparison = self.comparison;
        let times = self.times;

        quote! {
            {
                let core: std::rc::Rc<std::sync::RwLock<#expected_mock>> = #context.generate();
                core.write().unwrap().#method(Box::new(#comparison), #times);
            }
        }
    }
}