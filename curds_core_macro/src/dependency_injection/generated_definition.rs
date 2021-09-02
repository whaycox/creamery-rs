use super::*;

pub struct GeneratedDefinition {
    implementation: Ident,
}

impl Parse for GeneratedDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            implementation: input.parse()?,
        })
    }
}

impl GeneratedDefinition {
    pub fn transient(self, definition: &StructDefinition) -> TokenStream {
        let implementation = self.implementation;
        let name = definition.name.clone();

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#implementation>> for #name {
                fn generate(&self) -> std::rc::Rc<#implementation> {
                    std::rc::Rc::<#implementation>::new(curds_core_abstraction::dependency_injection::Injected::<#name>::inject(self))
                }
            }
        }
    }

    pub fn singleton(self, definition: &StructDefinition) -> TokenStream {
        let implementation = self.implementation;
        let name = definition.name.clone();
        let library_name = super::library_name();

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<#implementation>> for #name {
                fn generate(&self) -> std::rc::Rc<#implementation> {
                    let type_id = std::any::TypeId::of::<std::rc::Rc<#implementation>>();
                    if !self.#library_name.borrow().contains_key(&type_id) {
                        let service = std::rc::Rc::<#implementation>::new(curds_core_abstraction::dependency_injection::Injected::<#name>::inject(self));

                        let mut service_update = self.#library_name.take();
                        service_update.insert(type_id, service);
                        self.#library_name.replace(service_update);
                    }
                    self.#library_name
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