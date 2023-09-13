use super::*;

pub struct WheyReturnGenerator {
    context: Ident,
    expected_mock: Path,
    method: Ident,
    generator: ExprClosure,
    times: TokenStream,
}

impl Parse for WheyReturnGenerator {
    fn parse(input: ParseStream) -> Result<Self> {
        let context: Ident = input.parse()?;
        input.parse::<Token![~]>()?;
        let expected_mock: Path = input.parse()?;
        input.parse::<Token![~]>()?;
        let method: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let generator: ExprClosure = input.parse()?;
        input.parse::<Token![,]>()?;
        let times: TokenStream = input.parse()?;

        Ok(WheyReturnGenerator {
            context,
            expected_mock,
            method,
            generator,
            times,
        })
    }
}

impl WheyReturnGenerator {
    pub fn quote(self) -> TokenStream {
        let context = self.context;
        let expected_mock = MockedTraitDefinition::generate_core_name(&self.expected_mock);
        let method = WheyMockCore::store_return(&self.method);
        let generator = self.generator;
        let times = self.times;

        quote! {
            {
                let core: std::rc::Rc<std::sync::RwLock<#expected_mock>> = #context.generate();
                core.write().unwrap().#method(Box::new(#generator), #times);
            }
        }
    }
}