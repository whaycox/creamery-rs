use super::*;

pub struct PipelineDefinition {
    pub message: Type,
    pub context: Type,
    stages: Vec<PipelineStage>,
}

impl Parse for PipelineDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let message: Type = input.parse()?;
        input.parse::<Token![~]>()?;
        let context: Type = input.parse()?;

        let mut stages: Vec<PipelineStage> = vec![PipelineStage::default()];
        if input.peek(Token![&]) {
            input.parse::<Token![&]>()?;
            let stage_content;
            braced!(stage_content in input);
            let parsed_stages: Punctuated<PipelineStage, Token![,]> = stage_content.parse_terminated(PipelineStage::parse)?;
            stages = parsed_stages
                .into_iter()
                .collect();
        }
        else if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            let return_total: Type = input.parse()?;
            stages = vec![return_total.into()];
        }

        Ok(PipelineDefinition {
            message,
            context,
            stages,
        })
    }
}

impl PipelineDefinition {
    pub fn return_type(&self) -> &Option<Type> { 
        &self.stages[self.stages.len() - 1].return_type() 
    }

    pub fn trait_tokens(&self, visibility: &Visibility, parent_trait: &Ident, base_name: &Ident) -> TokenStream {
        let trait_name = format_ident!("{}Handler", base_name);
        let message_type = &self.message;
        let message_return = match &self.return_type() {
            Some(message_output) => quote! { #message_output },
            None => quote! { () }
        };

        quote! {
            #visibility trait #trait_name {
                fn handle(&self, dispatch: &dyn #parent_trait, input: &#message_type) -> curds_core_abstraction::message_dispatch::Result<#message_return>;
            }
        }
    }

    pub fn implementation_tokens(&self, base_name: &Ident) -> TokenStream {
        let context = &self.context;
        let mut stage_implementations: Vec<TokenStream> = Vec::new();
        let mut previous_stage_output: Option<Ident> = None;
        for stage in &self.stages {
            let trait_name = stage.trait_name(&base_name);
            let input_token = match &previous_stage_output {
                Some(name) => quote! { #name },
                None => quote! { message },
            };
            let assign_token = match &stage.return_type {
                Some(_return_type) => {
                    previous_stage_output = Some(trait_name.clone());
                    quote! { let #trait_name = }
                },
                None => {
                    previous_stage_output = None;
                    quote! { }
                },
            };
            stage_implementations.push(quote! {
                #assign_token <#context as #trait_name>::handle(&context, self, &#input_token)?;
            });
        }
        stage_implementations.push(match previous_stage_output {
            Some(name) => quote! { Ok(#name) },
            None => quote! { Ok(()) },
        });

        quote! {
            let context = curds_core_abstraction::dependency_injection::ServiceGenerator::<#context>::generate(self);
            #(#stage_implementations)*
        }
    }
}