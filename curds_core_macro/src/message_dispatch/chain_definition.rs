use super::*;

#[derive(Clone)]
pub struct ChainDefinition {
    stages: Vec<Ident>,
    return_type: Option<Type>,
}

impl Parse for ChainDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![|]>()?;
        let chain_content;
        bracketed!(chain_content in input);
        let stages: Punctuated<Ident, Token![,]> = chain_content.parse_terminated(Ident::parse)?;
        let return_type = if input.peek(Token![->]) {
            println!("Testing");
            input.parse::<Token![->]>()?;
            Some(input.parse::<Type>()?)
        }
        else {
            None
        };
        Ok(Self {
            stages: stages
                .into_iter()
                .collect(),
            return_type: return_type,
        })
    }
}

impl ChainDefinition {
    pub fn return_tokens(&self) -> TokenStream { 
        match self.return_type.clone() {
            Some(return_type) => quote! { std::option::Option<#return_type> },
            None => quote! { std::option::Option<()> },
        }
    }

    pub fn implementation_tokens(self, base_name: &Ident, context_type: &Type) -> TokenStream {
        let mut stage_implementations: Vec<TokenStream> = Vec::new();
        for stage in self.stages {
            let trait_name = format_ident!("{}{}", base_name, stage.clone());
            stage_implementations.push(quote! {
                let #stage = <#context_type as #trait_name>::handle(&context, self, &message)?;
                if #stage.is_some() {
                    return Ok(#stage);
                }
            });
        }
        stage_implementations.push(quote! { Ok(None) });
        quote! { #(#stage_implementations)* }       
    }

    pub fn trait_tokens(self, visibility: &Visibility, parent_trait: &Ident, base_name: &Ident, message_type: &Type) -> TokenStream {
        let mut stage_traits: Vec<TokenStream> = Vec::new();
        for stage in self.stages {
            let trait_name = format_ident!("{}{}", base_name, stage.clone());
            let return_token = match self.return_type.clone() {
                Some(return_type) => {
                    quote! { #return_type }
                },
                None => quote! { () },
            };
            stage_traits.push(quote! {                    
                #visibility trait #trait_name {
                    fn handle(&self, dispatch: &dyn #parent_trait, input: &#message_type) -> curds_core_abstraction::message_dispatch::Result<std::option::Option<#return_token>>;
                }
            });
        }
        quote! { #(#stage_traits)* }
    }
}