use super::*;

pub struct WheyTest {
    item: ItemFn,
}

impl WheyTest {
    pub fn quote(self) -> TokenStream {
        let attrs = self.item.attrs;
        let vis = self.item.vis;
        let mut sig = self.item.sig;
        let inputs = sig.inputs.clone();
        let mut injected_inputs: Vec<TokenStream> = Vec::new();
        for input in inputs {
            match input {
                FnArg::Typed(typed) => {
                    let name = typed.pat;
                    let injected_type = typed.ty;
                    injected_inputs.push(quote! {
                        let mut #name: #injected_type = #injected_type::construct();
                    });
                },
                _ => panic!("Unexpected input"),
            }
        }
        sig.inputs.clear();
        let block_pieces = self.item.block.stmts;
        quote! {
            #[test]
            #(#attrs)*
            #vis #sig {
                #(#injected_inputs)*
                #(#block_pieces)*
            }
        }
    }
}

impl Parse for WheyTest {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(WheyTest {
            item: input.parse()?,
        })
    }
}