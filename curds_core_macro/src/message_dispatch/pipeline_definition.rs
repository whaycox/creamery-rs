use super::*;

const HANDLER_NAME: &str = "Handler";

#[derive(Clone)]
pub struct PipelineDefinition {
    stages: Vec<(Ident, Option<Type>)>,
}
impl Default for PipelineDefinition {
    fn default() -> Self {
        Self { 
            stages: vec![(Ident::new(HANDLER_NAME, Span::call_site()), None)],
        }
    }
}
impl PipelineDefinition {
    pub fn new(return_type: Type) -> Self {
        Self {
            stages: vec![(Ident::new(HANDLER_NAME, Span::call_site()), Some(return_type))]
        }
    }

    pub fn implementation_tokens(self, base_name: &Ident, context_type: &Type) -> TokenStream {
        let stage_implementations: Vec<TokenStream> = self.stages
            .into_iter()
            .map(|stage| {
                let stage_name = stage.0;
                let trait_name = format_ident!("{}{}", base_name, stage_name.clone());
                let assign_token = match stage.1 {
                    Some(return_type) => quote! { let #stage_name = },
                    None => quote! { },
                };
                quote! {
                    #assign_token <#context_type as #trait_name>::handle(&context, self, message)?;
                }
            })
            .collect();
        quote! { #(#stage_implementations)* }       
    }

    pub fn trait_tokens(self, visibility: &Visibility, parent_trait: &Ident, base_name: &Ident, message_type: &Type) -> TokenStream {
        let stage_traits: Vec<TokenStream> = self.stages
            .into_iter()
            .map(|stage| {
                let trait_name = format_ident!("{}{}", base_name, stage.0);
                let return_token = match stage.1 {
                    Some(return_type) => quote! { #return_type },
                    None => quote! { () },
                };
                quote! {
                    #visibility trait #trait_name {
                        fn handle(&self, dispatch: &dyn #parent_trait, message: #message_type) -> curds_core_abstraction::message_dispatch::Result<#return_token>;
                    }
                }
            })
            .collect();
        quote! { #(#stage_traits)* }
    }
}

impl Parse for PipelineDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        todo!("pipeline parsing")
    }
}