use super::*;

#[derive(Clone, Debug)]
pub struct ChainDefinition {
    parameters: RoutingParameters,
    route: Option<ParallelRoute>,
}

impl ChainDefinition {
    pub fn new(parameters: RoutingParameters, route: Option<ParallelRoute>) -> Self {
        Self {
            parameters: parameters,
            route: route,
        }
    }

    pub fn context_type(&self) -> Type { self.parameters.context_type.clone() }

    pub fn expand(self, defaults: &DispatchDefaults) -> Self {        
        Self {
            parameters: self.parameters,
            route: match self.route {
                Some(route) => Some(route),
                None => Some(defaults.expand_chain()),
            },
        }
    }

    pub fn signature_tokens(self) -> TokenStream {
        let message_type = self.parameters.message_type;
        let response_tokens = match self.parameters.response_type {
            Some(response_type) => quote! { #response_type },
            None => quote! { () },
        };

        quote! { (&self, message: #message_type) -> curds_core_abstraction::message_dispatch::Result<std::option::Option<#response_tokens>> }
    }

    pub fn quote(self, base_name: &Ident) -> TokenStream {
        let mut stage_implementations: Vec<TokenStream> = Vec::new();
        let context_type = self.parameters.context_type;
        let route = self.route.unwrap();
        for stage in route.channels {
            let trait_name = format_ident!("{}{}", base_name, stage);
            stage_implementations.push(quote! {
                let #stage = <#context_type as #trait_name>::handle(&context, self, &message)?;
                if #stage.is_some() {
                    return Ok(#stage);
                }
            });
        }
        stage_implementations.push(quote! { Ok(None) });
        quote! {            
            let context = curds_core_abstraction::dependency_injection::ServiceGenerator::<std::rc::Rc<#context_type>>::generate(self); 
            #(#stage_implementations)* 
        }       
    }

    pub fn trait_tokens(self, visibility: &Visibility, parent_trait: &Ident, base_name: &Ident) -> TokenStream {
        let mut stage_traits: Vec<TokenStream> = Vec::new();
        let message_type = self.parameters.message_type;
        let return_token = match self.parameters.response_type.clone() {
            Some(return_type) => {
                quote! { #return_type }
            },
            None => quote! { () },
        };
        let route = self.route.unwrap();
        for stage in route.channels {
            let trait_name = format_ident!("{}{}", base_name, stage);            
            stage_traits.push(quote! {                    
                #visibility trait #trait_name {
                    fn handle(&self, dispatch: &dyn #parent_trait, input: &#message_type) -> curds_core_abstraction::message_dispatch::Result<std::option::Option<#return_token>>;
                }
            });
        }
        quote! { #(#stage_traits)* }
    }
}