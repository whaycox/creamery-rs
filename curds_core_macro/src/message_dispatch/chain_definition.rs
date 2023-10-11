use super::*;

pub struct ChainDefinition {
    pub message: Type,
    pub mutable: bool,
    pub context: Type,
    stages: Vec<ChainStage>,
    return_type: Option<Type>,
}

impl Parse for ChainDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let message: Type = input.parse()?;
        input.parse::<Token![~]>()?;
        let mut mutable = false;
        if input.peek(Token![mut]) {
            input.parse::<Token![mut]>()?;
            mutable = true;
        }
        let context: Type = input.parse()?;

        let mut stages: Vec<ChainStage> = vec![ChainStage::default()];
        input.parse::<Token![|]>()?;
        let stage_content;
        bracketed!(stage_content in input);
        let parsed_stages: Punctuated<ChainStage, Token![,]> = stage_content.parse_terminated(ChainStage::parse)?;
        stages = parsed_stages
            .into_iter()
            .collect();

        let mut return_type: Option<Type> = None;
        if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            return_type = Some(input.parse()?);
        }

        Ok(Self {
            message,
            mutable,
            context,
            stages,
            return_type,
        })
    }
}

impl ChainDefinition {
    pub fn apply_template(self, defaults: &MessageDefaults) -> Self {
        self
    }

    pub fn has_return(&self) -> bool { self.return_type.is_some() }
    pub fn return_type(&self, error_type: &Type) -> TokenStream { 
        match &self.return_type {
            Some(output) => quote! { std::option::Option<std::result::Result<#output, #error_type>> },
            None => quote! { std::option::Option<std::result::Result<(), #error_type>> },
        } 
    }
    pub fn mock_return_attribute(&self) -> TokenStream { 
        if self.return_type.is_none() {
            quote! { #[mock_default_return(|_| Some(Ok(())))] }
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
        let message_type = &self.message;
        let return_type = self.return_type(&message_trait.error_type);

        let mut stage_traits: Vec<TokenStream> = Vec::new();
        let stage_length = self.stages.len();
        for i in 0..stage_length {
            let stage = &self.stages[i];
            let trait_name = stage.trait_name(base_name);

            stage_traits.push(quote! {
                #visibility trait #trait_name {
                    fn handle(#receiver_token, dispatch: #dyn_token #parent_trait, input: &#message_type) -> #return_type;
                }
            });
        }

        quote! { #(#stage_traits)* }
    }
    
    pub fn implementation_tokens(&self, base_name: &Ident) -> TokenStream {
        let context = &self.context;
        let mut stage_implementations: Vec<TokenStream> = Vec::new();

        let stage_length = self.stages.len();
        for i in 0..stage_length {
            let stage = &self.stages[i];
            let stage_name = &stage.name;
            let trait_name = stage.trait_name(&base_name);
            let context_input = if self.mutable {
                quote! { &mut context }
            }
            else {
                quote! { &context }
            };
            stage_implementations.push(quote! {
                let #stage_name = <#context as #trait_name>::handle(#context_input, self, &message);
                if #stage_name.is_some() { return #stage_name }
            });
        }
        stage_implementations.push(quote! { None });

        quote! {
            let mut context = curds_core_abstraction::dependency_injection::ServiceGenerator::<#context>::generate(self);
            #(#stage_implementations)*
        }
    }
}