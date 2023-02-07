use super::*;

pub struct WheyExpectation {
    context: Ident,
    mocked_trait: Path,
    expected_call: Ident,
    expected_values: Punctuated<Expr, Token![,]>,
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
        let mut closure_inputs: Vec<Ident> = Vec::new();
        for i in 0..expected_values.len() {
            let input_ident = format!("v{}", i);
            closure_inputs.push(Ident::new(&input_ident, Span::call_site()));
        }
        let times = self.times;

        quote! {
            {
                let core: std::rc::Rc<#core_name> = #context.generate();
                core.#expect_ident(Box::new(|#(#closure_inputs),*| (#(#closure_inputs),*) == (#expected_values)), #times);
            }
        }
    }
}