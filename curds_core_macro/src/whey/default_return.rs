use super::*;

pub struct WheyDefaultReturn {
    context: Ident,
    expected_mock: Path,
    method: Ident,
    generator: ExprClosure,
}

impl Parse for WheyDefaultReturn {
    fn parse(input: ParseStream) -> Result<Self> {
        let context: Ident = input.parse()?;
        input.parse::<Token![~]>()?;
        let expected_mock: Path = input.parse()?;
        input.parse::<Token![~]>()?;
        let method: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let generator: ExprClosure = input.parse()?;

        Ok(WheyDefaultReturn {
            context,
            expected_mock,
            method,
            generator,
        })
    }
}

impl WheyDefaultReturn {
    pub fn quote(self) -> TokenStream {
        let context = self.context;
        let expected_mock = MockedTraitDefinition::generate_core_name(&self.expected_mock);
        let method = WheyMockCore::default_return(&self.method);
        let generator = self.generator;
        quote! {
            {
                let core: std::rc::Rc<std::sync::RwLock<#expected_mock>> = #context.generate();
                core.write().unwrap().#method(Box::new(#generator));
            }
        }
    }
}