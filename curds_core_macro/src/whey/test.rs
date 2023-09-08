use super::*;

pub struct WheyTest {
    item: ItemFn,
}

impl WheyTest {
    pub fn quote(self, context: WheyTestContext) -> TokenStream {
        let attrs = self.item.attrs;
        let vis = self.item.vis;
        let sig = self.item.sig;
        let context_type = context.quote();
        let block_pieces = self.item.block.stmts;
        quote! {
            #[test]
            #(#attrs)*
            #vis #sig {
                #context_type
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