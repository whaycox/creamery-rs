use super::*;

pub struct WheySequence {
    context: Ident,
    stages: Vec<WheySequenceStage>,
}

impl Parse for WheySequence {
    fn parse(input: ParseStream) -> Result<Self> {
        let context: Ident = input.parse()?;
        input.parse::<Token![~]>()?;
        let sequence_content;
        bracketed!(sequence_content in input);
        let sequence_stages: Punctuated<WheySequenceStage, Token![,]> = sequence_content.parse_terminated(WheySequenceStage::parse)?;

        Ok(Self {
            context,
            stages: sequence_stages.into_iter().collect(),
        })
    }
}

impl WheySequence {
    pub fn quote(self) -> TokenStream {
        let context = &self.context;
        let stage_tokens: Vec<TokenStream> = self.stages
            .into_iter()
            .map(|stage| stage.quote(context))
            .collect();

        quote! {
            #(#stage_tokens)*
        }
    }
}