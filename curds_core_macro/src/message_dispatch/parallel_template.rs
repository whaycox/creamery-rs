use proc_macro2::Spacing;

use super::*;

pub struct ParallelTemplate {
    channels: Vec<Ident>,
}
impl Default for ParallelTemplate {
    fn default() -> Self { 
        Self {
            channels: vec![ Ident::new(HANDLER_NAME, Span::call_site()) ]
        } 
    }
}

impl Parse for ParallelTemplate {
    fn parse(input: ParseStream) -> Result<Self> {
        todo!("ChainTemplate parse")
    }
}

impl ParallelTemplate {
    pub fn expand(&self) -> ParallelRoute {
        ParallelRoute::new(self.channels.clone())
    }
}