use super::*;

pub struct InjectedDefinition {
    visibility: Option<Token![pub]>,
    definition: DependencyDefinition,
    defaults: Vec<DefaultedFields>,
    injected_implmentation: InjectedImplementation,
}
impl InjectedDefinition {
    pub fn quote(self) -> TokenStream {
        let visibility = self.visibility;
        let ident = self.definition.ident;
        let fields = self.definition.fields
            .clone()
            .into_iter();
        let injected_implementation = self.injected_implmentation.quote(self.defaults);

        quote! {
            #visibility struct #ident {
                #(#fields),*
            }

            #injected_implementation
        }
    }
}

impl Parse for InjectedDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let defaults = DefaultedFields::parse_defaults(input)?;
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
        let injected_implementation = InjectedImplementation::new(definition.clone());

        Ok(InjectedDefinition {
            visibility: visibility,
            definition: definition,
            defaults: defaults,
            injected_implmentation: injected_implementation,
        })
    }
}