use super::*;

pub struct WheyContextSequence {
    stages: Vec<WheySequenceStage>,
}

impl Parse for WheyContextSequence {
    fn parse(input: ParseStream) -> Result<Self> {
        let sequence_content;
        bracketed!(sequence_content in input);
        let sequence_stages: Punctuated<WheySequenceStage, Token![,]> = sequence_content.parse_terminated(WheySequenceStage::parse)?;

        Ok(Self {
            stages: sequence_stages.into_iter().collect(),
        })
    }
}

impl WheyContextSequence {
    pub fn quote(self) -> TokenStream {
        let stage_tokens: Vec<TokenStream> = self.stages
            .into_iter()
            .map(|stage| stage.quote(None))
            .collect();

        quote! {
            #(#stage_tokens)*
        }
    }
}