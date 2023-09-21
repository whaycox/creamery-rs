use super::*;

pub enum WheyExpectation {
    Closure(ExprClosure),
    Delegate(Ident),
}

impl Parse for WheyExpectation {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(closure) = input.parse::<ExprClosure>() {
            return Ok(WheyExpectation::Closure(closure));
        } 
        else {
            return Ok(WheyExpectation::Delegate(input.parse()?));
        }
    }
}

impl WheyExpectation {
    pub fn quote(&self) -> TokenStream {
        match self {
            WheyExpectation::Closure(closure) => quote! { #closure },
            WheyExpectation::Delegate(delegate) => quote! { #delegate },
        }
    }
}

impl ToTokens for WheyExpectation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.quote())
    }
}
