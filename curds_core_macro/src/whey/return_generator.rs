use super::*;

pub struct WheyReturnGenerator {
    context: Option<Ident>,
    expected_mock: Path,
    method: Ident,
    generator: WheyExpectation,
    times: TokenStream,
}

impl Parse for WheyReturnGenerator {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut context: Option<Ident> = None;
        if let Ok(_) = input.parse::<Token![self]>() { } 
        else {
            context = Some(input.parse()?);
        }

        input.parse::<Token![~]>()?;
        let expected_mock: Path = input.parse()?;
        input.parse::<Token![~]>()?;
        let method: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let generator: WheyExpectation = input.parse()?;
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
        let context = match self.context {
            Some(ident) => quote! { #ident },
            None => quote! { self }
        };
        let expected_mock = MockedTraitDefinition::generate_testing_name(&self.expected_mock);
        let method = WheyMock::store_return(&self.method);
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