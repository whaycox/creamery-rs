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
        let load_core = WheyMockCore::expect_input(&self.mocked_trait.get_ident().unwrap());
        let load_expectation = WheyMockCore::expect_input(&self.expected_call);
        let expected_values = self.expected_values;
        let mut loaded_values = vec![quote! { (#expected_values) }];
        match self.return_value {
            Some(expected_return) => loaded_values.push(quote! { Some(#expected_return) }),
            None => loaded_values.push(quote! { Some(()) }),
        };
        let times = self.times;
        loaded_values.push(quote! { #times });

        quote! {
            {
                let mut core = #context.#load_core();
                core.#load_expectation(#(#loaded_values),*);
            }
        }
    }
}