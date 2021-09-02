use super::*;

pub struct MappedDefinition {
    abstraction: Ident,
    implementation: Ident,
}

impl Parse for MappedDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let abstraction: Ident = input.parse()?;
        input.parse::<Token![<-]>()?;
        let implementation: Ident = input.parse()?;

        Ok(Self {
            abstraction: abstraction,
            implementation: implementation,
        })
    }
}

impl MappedDefinition {
    pub fn transient(self, definition: &StructDefinition) -> TokenStream {
        let abstraction = self.abstraction;
        let implementation = self.implementation;
        let name = definition.name.clone();

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<dyn #abstraction>> for #name {
                fn generate(&self) -> std::rc::Rc<dyn #abstraction> {
                    std::rc::Rc::<#implementation>::new(curds_core_abstraction::dependency_injection::Injected::<#name>::inject(self))
                }
            }
        }
    }

    pub fn singleton(self, definition: &StructDefinition) -> TokenStream {
        let abstraction = self.abstraction;
        let implementation = self.implementation;
        let name = definition.name.clone();
        let library_name = super::library_name();

        quote! {
            impl curds_core_abstraction::dependency_injection::ServiceGenerator<std::rc::Rc<dyn #abstraction>> for #name {
                fn generate(&self) -> std::rc::Rc<dyn #abstraction> {
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