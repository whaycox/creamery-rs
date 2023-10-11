use super::*;

pub struct PipelineDefinition {
    pub message: Type,
    pub mutable: bool,
    pub context: Type,
    stages: Vec<PipelineStage>,
}

impl Parse for PipelineDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let message: Type = input.parse()?;
        input.parse::<Token![~]>()?;
        let mut mutable = false;
        if input.peek(Token![mut]) {
            input.parse::<Token![mut]>()?;
            mutable = true;
        }
        let context: Type = input.parse()?;

        let mut stages: Vec<PipelineStage> = vec![PipelineStage::default()];
        if input.peek(Token![&]) {
            input.parse::<Token![&]>()?;
            if input.peek(token::Brace) {
                let stage_content;
                braced!(stage_content in input);
                let parsed_stages: Punctuated<PipelineStage, Token![,]> = stage_content.parse_terminated(PipelineStage::parse)?;
                stages = parsed_stages
                    .into_iter()
                    .collect();
            }
        }
        if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            let return_total: Type = input.parse()?;
            stages = vec![return_total.into()];
        }

        Ok(PipelineDefinition {
            message,
            mutable,
            context,
            stages,
        })
    }
}

impl PipelineDefinition {
    pub fn apply_template(self, defaults: &MessageDefaults) -> Self {
        match &defaults.pipeline {
            Some(pipeline_default) => {
                Self {
                    message: self.message,
                    mutable: self.mutable,
                    context: self.context,
                    stages: pipeline_default.stages(self.stages[self.stages.len() - 1].return_type())
                }
            },
            None => self,
        }
    }

    pub fn return_type(&self, error_type: &Type) -> TokenStream {
        match self.stages[self.stages.len() - 1].return_type() {
            StageReturn::None => quote! { std::result::Result<(), #error_type> },
            StageReturn::Explicit(output) => quote! { std::result::Result<#output, #error_type> },
            StageReturn::Message => {
                let message = &self.message;
                quote! { std::result::Result<#message, #error_type> }
            }
        }
    }
    pub fn mock_return_attribute(&self) -> TokenStream { 
        if let StageReturn::None = self.stages[self.stages.len() - 1].return_type() {
            quote! { #[mock_default_return(|_| Ok(()))] }
        }
        else { quote! {} }
    }

    pub fn trait_tokens(&self, visibility: &Visibility, message_trait: &MessageTraitDefinition, base_name: &Ident) -> TokenStream {
        let receiver_token = if self.mutable {
            quote! { &mut self }
        }
        else {
            quote! { &self }
        };
        let dyn_token = if self.mutable {
            quote! { &mut dyn }
        }
        else {
            quote! { &dyn }
        };
        let parent_trait = &message_trait.name;
        let error_type = &message_trait.error_type;
        let mut stage_traits: Vec<TokenStream> = Vec::new();
        let mut previous_stage_output: Option<Type> = None;
        let message_type = &self.message;

        let stage_length = self.stages.len();
        for i in 0..stage_length {
            let stage = &self.stages[i];
            let trait_name = stage.trait_name(base_name);
            let mut input_token = match &previous_stage_output {
                Some(output) => quote! { #output },
                None => quote! { #message_type }
            };
            let return_token = match stage.return_type() {
                StageReturn::None => {
                    if i < stage_length - 1 {
                        input_token = quote! { &#input_token };
                    }
                    quote! { std::result::Result<(), #error_type> }
                },
                StageReturn::Explicit(return_type) => {
                    previous_stage_output = Some(return_type.clone());
                    quote! { std::result::Result<#return_type, #error_type> }
                },
                StageReturn::Message => {
                    let message = &self.message;
                    previous_stage_output = Some(message.clone());
                    quote! { std::result::Result<#message, #error_type> }
                }
            };

            stage_traits.push(quote! {
                #visibility trait #trait_name {
                    fn handle(#receiver_token, dispatch: #dyn_token #parent_trait, input: #input_token) -> #return_token ;
                }
            });
        }

        quote! { #(#stage_traits)* }
    }

    pub fn implementation_tokens(&self, base_name: &Ident) -> TokenStream {
        let context = &self.context;
        let mut stage_implementations: Vec<TokenStream> = Vec::new();
        let mut previous_stage_output: Option<Ident> = None;

        let stage_length = self.stages.len();
        for i in 0..stage_length {
            let stage = &self.stages[i];
            let trait_name = stage.trait_name(&base_name);
            let context_input = if self.mutable {
                quote! { &mut context }
            }
            else {
                quote! { &context }
            };
            let mut input_token = match &previous_stage_output {
                Some(name) => quote! { #name },
                None => quote! { message },
            };
            let assign_token = match &stage.return_type {
                StageReturn::Explicit(_) | StageReturn::Message => {
                    previous_stage_output = Some(stage.name.clone());
                    quote! { let #previous_stage_output = }
                },
                StageReturn::None => {
                    if i < stage_length - 1 {
                        input_token = quote! { &#input_token };
                    }
                    previous_stage_output = None;
                    quote! { }
                },
            };
            stage_implementations.push(quote! {
                #assign_token <#context as #trait_name>::handle(#context_input, self, #input_token)?;
            });
        }
        stage_implementations.push(match previous_stage_output {
            Some(name) => quote! { Ok(#name) },
            None => quote! { Ok(()) },
        });

        quote! {
            let mut context = curds_core_abstraction::dependency_injection::ServiceGenerator::<#context>::generate(self);
            #(#stage_implementations)*
        }
    }
}