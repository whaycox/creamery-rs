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
        let channels: Punctuated<Ident, Token![,]> = input.parse_terminated(Ident::parse)?;
        Ok(Self {
            channels: channels
                .into_iter()
                .collect(),
        })
    }
}

impl ParallelTemplate {
    pub fn expand(&self) -> ParallelRoute {
        ParallelRoute::new(self.channels.clone())
    }
}