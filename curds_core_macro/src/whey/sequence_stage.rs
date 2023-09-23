use super::*;

pub struct WheySequenceStage {
    expected_mock: Path,
    method: Ident,
    input_comparison: Option<WheyExpectation>,
    return_generator: Option<WheyExpectation>,
}

impl Parse for WheySequenceStage {
    fn parse(input: ParseStream) -> Result<Self> {
        let expected_mock: Path = input.parse()?;
        input.parse::<Token![~]>()?;
        let method: Ident = input.parse()?;

        let mut input_comparison: Option<WheyExpectation> = None;
        let input_content;
        parenthesized!(input_content in input);
        if !input_content.is_empty() {
            input_comparison = Some(input_content.parse()?);
        }
        
        let mut return_generator: Option<WheyExpectation> = None;
        if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            return_generator = Some(input.parse()?);
        }

        Ok(Self {
            expected_mock,
            method,
            input_comparison,
            return_generator,
        })
    }
}

impl WheySequenceStage {
    pub fn quote(self, context: Option<&Ident>) -> TokenStream {
        let context = match context {
            Some(ident) => quote! { #ident },
            None => quote! { self }
        };
        let mock_core = MockedTraitDefinition::generate_core_name(&self.expected_mock);
        let expected_mock = self.expected_mock;
        let method_str = format!("{}", self.method);
        let method = self.method;

        let sequence = quote! {
            {
                let synchronizer: std::rc::Rc<std::sync::RwLock<curds_core_abstraction::whey::WheySynchronizer>> = #context.generate();
                synchronizer.write().unwrap().load(std::any::TypeId::of::<#mock_core>(), String::from(#method_str));
            }
        };
        let input_comparison = match self.input_comparison {
            Some(comparison) => quote! { 
                curds_core_macro::mock_input!(#context ~ #expected_mock ~ #method, #comparison, 1);
            },
            None => quote! {},
        };
        let return_generator = match self.return_generator {
            Some(generator) => quote! {
                curds_core_macro::mock_return!(#context ~ #expected_mock ~ #method, #generator, 1);
            },
            None => quote! {},
        };

        quote! {
            #sequence
            #input_comparison
            #return_generator
        }
    }
}