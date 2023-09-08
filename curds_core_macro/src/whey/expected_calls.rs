use super::*;

pub struct WheyExpectedCalls {
    context: Ident,
    expected_mock: Path,
    method: Ident,
    times: TokenStream,
}

impl WheyExpectedCalls {
    pub fn quote(self) -> TokenStream {
        let context = self.context;
        let expected_mock = MockedTraitDefinition::generate_core_name(&self.expected_mock);
        let method = WheyMockCore::expect_calls(&self.method);
        let times = self.times;
        quote! {
            {
                let core: std::rc::Rc<std::sync::RwLock<#expected_mock>> = #context.generate();
                core.write().unwrap().#method(#times);
            }
        }
    }
}

impl Parse for WheyExpectedCalls {
    fn parse(input: ParseStream) -> Result<Self> {
        let context: Ident = input.parse()?;
        input.parse::<Token![~]>()?;
        let expected_mock: Path = input.parse()?;
        input.parse::<Token![~]>()?;
        let method: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let times: TokenStream = input.parse()?;

        Ok(WheyExpectedCalls {
            context,
            expected_mock,
            method,
            times,
        })
    }
}