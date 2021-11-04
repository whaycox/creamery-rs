use super::*;

#[derive(Clone)]
pub struct PipelineDefinition {
    stages: Vec<PipelineStage>,
}
impl Default for PipelineDefinition {
    fn default() -> Self {
        Self { 
            stages: vec![PipelineStage::default_message()],
        }
    }
}

impl Parse for PipelineDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![&]>()?;
        let pipeline_content;
        braced!(pipeline_content in input);
        let stages: Punctuated<PipelineStage, Token![,]> = pipeline_content.parse_terminated(PipelineStage::parse)?;
        Ok(Self {
            stages: stages
                .into_iter()
                .collect(),
        })
    }
}

impl PipelineDefinition {
    pub fn new(return_type: Type) -> Self {
        Self {
            stages: vec![PipelineStage::default_request(return_type)]
        }
    }

    pub fn return_tokens(&self) -> TokenStream {
        if self.stages.len() > 0 {
            let last = &self.stages[self.stages.len() - 1];
            match last.return_type() {
                Some(return_type) => quote! { #return_type },
                None => quote! { () },
            }
        }
        else {
            quote! { () }
        }
    }

    pub fn implementation_tokens(self, base_name: &Ident, context_type: &Type) -> TokenStream {
        let mut stage_implementations: Vec<TokenStream> = Vec::new();
        let mut previous_stage_output: Option<Ident> = None;
        for stage in self.stages {
            let stage_name = stage.name;
            let trait_name = format_ident!("{}{}", base_name, stage_name.clone());
            let input_token = match &previous_stage_output {
                Some(name) => quote! { #name },
                None => quote! { message },
            };
            let assign_token = match stage.return_type {
                Some(_return_type) => {
                    previous_stage_output = Some(stage_name.clone());
                    quote! { let #stage_name = }
                },
                None => {
                    previous_stage_output = None;
                    quote! { }
                },
            };
            stage_implementations.push(quote! {
                #assign_token <#context_type as #trait_name>::handle(&context, self, &#input_token)?;
            });
        }
        stage_implementations.push(match previous_stage_output {
            Some(name) => quote! { Ok(#name) },
            None => quote! { Ok(()) },
        });
        quote! { #(#stage_implementations)* }       
    }

    pub fn trait_tokens(self, visibility: &Visibility, parent_trait: &Ident, base_name: &Ident, message_type: &Type) -> TokenStream {
        let mut stage_traits: Vec<TokenStream> = Vec::new();
        let mut previous_stage_output: Option<Type> = None;
        for stage in self.stages {
            let stage_name = stage.name;
            let trait_name = format_ident!("{}{}", base_name, stage_name.clone());
            let input_token = match &previous_stage_output {
                Some(output) => quote! { #output },
                None => quote! { #message_type },
            };
            let return_token = match stage.return_type {
                Some(return_type) => {
                    previous_stage_output = Some(return_type.clone());
                    quote! { #return_type }
                },
                None => quote! { () },
            };
            stage_traits.push(quote! {                    
                #visibility trait #trait_name {
                    fn handle(&self, dispatch: &dyn #parent_trait, input: &#input_token) -> curds_core_abstraction::message_dispatch::Result<#return_token>;
                }
            });
        }
        quote! { #(#stage_traits)* }
    }
}