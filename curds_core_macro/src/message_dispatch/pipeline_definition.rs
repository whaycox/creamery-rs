use super::*;

#[derive(Clone, Debug)]
pub struct PipelineDefinition {
    parameters: RoutingParameters,
    route: Option<SerialRoute>,
}
impl PipelineDefinition {
    pub fn explicit(parameters: RoutingParameters, route: SerialRoute) -> Self {
        Self {
            parameters: parameters,
            route: Some(route),
        }
    }
    
    pub fn implicit(parameters: RoutingParameters) -> Self {
        Self {
            parameters: parameters,
            route: None,
        }
    }

    pub fn context_type(&self) -> Type { self.parameters.context_type.clone() }

    pub fn expand(self, defaults: &DispatchDefaults) -> Self {
        Self {
            parameters: self.parameters.clone(),
            route: match self.route {
                Some(route) => Some(route),
                None => Some(defaults.expand_pipeline(self.parameters)),
            },
        }
    }

    pub fn quote(self, base_name: &Ident) -> TokenStream {
        let context_type = self.parameters.context_type;
        let mut stage_implementations: Vec<TokenStream> = Vec::new();
        let mut previous_stage_output: Option<Ident> = None;
        let route = self.route.unwrap();
        for stage in route.stages {
            let trait_name = stage.trait_name(&base_name);
            let input_token = match &previous_stage_output {
                Some(name) => quote! { #name },
                None => quote! { message },
            };
            let assign_token = match stage.output {
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
                #assign_token <#context_type as #trait_name>::handle(&context, self, &#input_token)?;
            });
        }
        stage_implementations.push(match previous_stage_output {
            Some(name) => quote! { Ok(#name) },
            None => quote! { Ok(()) },
        });
        quote! {
            let context = curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<#context_type>>::generate(self);
            #(#stage_implementations)* 
        } 
    }

    pub fn signature_tokens(self) -> TokenStream {
        let message_type = self.parameters.message_type;
        let response_tokens = match self.parameters.response_type {
            Some(response_type) => quote! { #response_type },
            None => quote! { () },
        };

        quote! { (&self, message: #message_type) -> curds_core_abstraction::message_dispatch::Result<#response_tokens> }
    }

    pub fn trait_tokens(self, visibility: &Visibility, parent_trait: &Ident, base_name: &Ident) -> TokenStream {
        let mut stage_traits: Vec<TokenStream> = Vec::new();
        let mut previous_stage_output: Option<Type> = None;

        let message_type = self.parameters.message_type;
        let route = self.route.unwrap();
        for stage in route.stages {
            let trait_name = stage.trait_name(base_name);
            let input_token = match &previous_stage_output {
                Some(output) => quote! { #output },
                None => quote! { #message_type }
            };       
            let return_token = match stage.output() {
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