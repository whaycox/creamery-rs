use super::*;

#[derive(Clone, Debug)]
pub struct ParallelRoute {
    pub channels: Vec<Ident>,
}
impl ParallelRoute {
    pub fn new(channels: Vec<Ident>) -> Self {
        Self {
            channels: channels,
        }
    }
}

impl Parse for ParallelRoute {
    fn parse(input: ParseStream) -> Result<Self> {
        let channel_content;
        bracketed!(channel_content in input);
        let channels: Punctuated<Ident, Token![,]> = channel_content.parse_terminated(Ident::parse)?;
        Ok(Self {
            channels: channels
                .into_iter()
                .collect(),
        })
    }
}