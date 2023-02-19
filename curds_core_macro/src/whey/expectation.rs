use super::*;

pub struct WheyExpectation {
    context: Ident,
    mocked_trait: Path,
    expected_call: Ident,
    expected_values: Punctuated<Expr, Token![,]>,
    return_value: Option<Expr>,
    times: Expr,
}

impl Parse for WheyExpectation {
    fn parse(input: ParseStream) -> Result<Self> {
        let context: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        input.parse::<Option<Token![dyn]>>()?;
        let mocked_trait: Path = input.parse()?;
        input.parse::<Token![.]>()?;
        let expected_call: Ident = input.parse()?;
        let value_content;
        parenthesized!(value_content in input);        
        let expected_values: Punctuated::<Expr, Token![,]> = value_content.parse_terminated(Expr::parse)?;
        let mut return_value: Option<Expr> = None;
        if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            return_value = Some(input.parse()?);
        }
        let times: Expr = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            input.parse()?
        }
        else {
            syn::parse2(quote! { 1 })?
        };
        
        Ok(WheyExpectation { 
            context, 
            mocked_trait,
            expected_call,
            expected_values,
            return_value,
            times,
        })
    }
}

impl WheyExpectation {
    pub fn quote(self) -> TokenStream {
        let context = self.context;
        let core_name = WheyMockedType::generate_core_name(&self.mocked_trait);
        let expect_ident = WheyMockCore::expect_ident(&self.expected_call);
        let expected_values = self.expected_values;
        let times = self.times;
        let expect_return = match self.return_value {
            Some(expected_return) => {
                let expect_return_ident = WheyMockCore::expect_return_ident(&self.expected_call);
                quote! {
                    mutable_core.#expect_return_ident((#expected_return), #times);
                }
            },
            None => quote! {},
        };

        quote! {
            {
                let mut core: std::rc::Rc<#core_name> = #context.generate();
                let mut mutable_core = std::rc::Rc::get_mut(&mut core).unwrap();
                mutable_core.#expect_ident((#expected_values), #times);
                #expect_return
            }
        }
    }
}