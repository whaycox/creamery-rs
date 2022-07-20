use super::*;

pub struct SingletonCollection {
    consumed: HashSet<Ident>,
    singletons: HashMap<Type, SingletonIdentifier>,
}

impl SingletonCollection {
    pub fn new() -> Self {
        Self { 
            consumed: HashSet::new(),
            singletons: HashMap::new(), 
        }
    }

    pub fn singleton(&self, ty: &Type) -> Ident { self.singletons[ty].ident() }
    pub fn is_singleton(&self, ident: &Ident) -> bool { self.consumed.contains(ident) }

    pub fn register_singletons(&mut self, library: &Vec<ServiceProduction>) -> Result<()> {
        for production in library {
            match production {
                ServiceProduction::GenerateSingleton(definition) => self.register_singleton(definition.singleton_type(), false),
                ServiceProduction::ForwardSingleton(definition) => self.register_singleton(definition.singleton_type(), definition.trait_storage()),
                _ => continue,
            }
        }
        Ok(())
    }
    fn register_singleton(&mut self, ty: Type, trait_storage: bool) {
        if !self.singletons.contains_key(&ty) {
            let singleton = self.generate_singleton(trait_storage);
            self.singletons.insert(ty, singleton);
        }
    }
    fn generate_singleton(&mut self, trait_storage: bool) -> SingletonIdentifier {
        let mut generated = SingletonIdentifier::new(trait_storage);
        while self.consumed.contains(&generated.ident()) {
            generated = SingletonIdentifier::new(trait_storage);
        }
        self.consumed.insert(generated.ident());

        generated
    }

    pub fn add_singletons(&self, item: &mut ItemStruct) -> Result<()> {
        for singleton in &self.singletons {
            Self::add_singleton(item, singleton.1, singleton.0)?;
        }

        Ok(())
    }
    fn add_singleton(item: &mut ItemStruct, singleton: &SingletonIdentifier, ty: &Type) -> Result<()> {
        match &mut item.fields {
            Fields::Named(named) => {
                let singleton_type: Type = syn::parse2(if singleton.trait_storage {
                    quote! {
                        std::cell::RefCell<std::option::Option<std::rc::Rc<dyn #ty>>>
                    }
                }
                else {
                    quote! {
                        std::cell::RefCell<std::option::Option<std::rc::Rc<#ty>>>
                    }
                })?;
                let mut singleton_field = Field {
                    attrs: Default::default(),
                    vis: Visibility::Inherited,
                    ident: Some(singleton.ident()),
                    colon_token: None,
                    ty: singleton_type,
                };
                let defaulted_attribute: Attribute = Attribute { 
                    pound_token: Default::default(), 
                    style: AttrStyle::Outer, 
                    bracket_token: Default::default(), 
                    path: syn::parse2(quote! { defaulted })?, 
                    tokens: TokenStream::new(), 
                };
                singleton_field.attrs.push(defaulted_attribute);
                named.named.push(singleton_field);
            },
            _ => panic!("Only named fields are supported"),
        }

        Ok(())
    }
}