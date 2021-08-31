use super::*;

pub struct SingletonGenerateDefinition {
    implementation: Ident,
}
impl SingletonGenerateDefinition {
    pub fn quote(self, definition: &DependencyDefinition) -> TokenStream {
        let implementation = self.implementation;
        let ident = definition.ident.clone();
        let library_ident = Ident::new(SERVICES_LIBRARY_NAME, Span::call_site());

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#implementation>> for #ident {
                fn generate(&self) -> std::rc::Rc<#implementation> {
                    let type_id = std::any::TypeId::of::<std::rc::Rc<#implementation>>();
                    if !self.#library_ident.borrow().contains_key(&type_id) {
                        let service = std::rc::Rc::<#implementation>::new(curds_core_abstraction::dependency_injection::Injected::<#ident>::inject(self));

                        let mut service_update = self.#library_ident.take();
                        service_update.insert(type_id, service);
                        self.#library_ident.replace(service_update);
                    }
                    self.#library_ident
                        .borrow()
                        .get(&type_id)
                        .unwrap()
                        .clone()
                        .downcast::<#implementation>()
                        .unwrap()                    
                }
            }
        }
    }
}

impl Parse for SingletonGenerateDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            implementation: input.parse()?,
        })
    }
}