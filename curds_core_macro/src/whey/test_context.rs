use super::*;

pub struct WheyTestContext {
    context: Type,
    name: Ident,
}

impl WheyTestContext {
    pub fn quote(self) -> TokenStream {
        let context = self.context;
        let name = self.name;
        quote! {
            let mut #name = #context::construct();
        }
    }
}

impl Parse for WheyTestContext {
    fn parse(input: ParseStream) -> Result<Self> {
        let context: Type = input.parse()?;
        input.parse::<Token![~]>()?;
        let name: Ident = input.parse()?;

        Ok(WheyTestContext {
            context,
            name,
        })
    }
}