use super::*;

pub struct ServiceProviderDefinition {
    library: Vec<ServiceProduction>,
    visibility: Option<Token![pub]>,
    definition: DependencyDefinition,
    injected: InjectedImplementation,
}
impl ServiceProviderDefinition {
    pub fn quote(self) -> TokenStream {
        let visibility = self.visibility;
        let definition = self.definition.clone();
        let ident = definition.ident;
        let fields = definition.fields;
        let injected = self.injected.quote();
        let library_definition = self.definition.clone();
        let library = self.library
            .into_iter()
            .map(|production| production.quote(&library_definition));

        quote! {
            #visibility struct #ident {
                #fields
            }
    
            #injected

            #(#library)*
        }
    }
}


impl Parse for ServiceProviderDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let library = ServiceProduction::parse(input)?;
        let visibility: Option<Token![pub]> = input.parse()?;
        input.parse::<Token![struct]>()?;
        let ident: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let fields = content.parse_terminated(Field::parse_named)?;
        let definition = DependencyDefinition {
            ident: ident,
            fields: fields,
        };
        let injected = InjectedImplementation::new(definition.clone());

        Ok(ServiceProviderDefinition {
            library: library,
            visibility: visibility,
            definition: definition,
            injected: injected,
        })
    }
}