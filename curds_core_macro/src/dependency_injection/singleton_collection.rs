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

    pub fn register_singletons(&mut self, item: &ItemStruct, library: &Vec<ServiceProduction>) -> Result<()> {
        for production in library {
            match production {
                ServiceProduction::GenerateSingleton(definition) => self.register_singleton(definition.singleton_description()),
                ServiceProduction::ForwardSingleton(definition) => if definition.is_promoted() { self.register_singleton(definition.singleton_description()) },
                _ => continue,
            }
        }
        Ok(())
    }
    fn register_singleton(&mut self, description: SingletonDescription) {
        if !self.singletons.contains_key(&description.requested) {
            let singleton = self.generate_singleton(&description);
            self.singletons.insert(description.requested, singleton);
        }
    }
    fn generate_singleton(&mut self, description: &SingletonDescription) -> SingletonIdentifier {
        let mut generated = SingletonIdentifier::new(description);
        while self.consumed.contains(&generated.ident()) {
            generated = SingletonIdentifier::new(description);
        }
        self.consumed.insert(generated.ident());

        generated
    }

    pub fn add_singletons(&self, item: &mut ItemStruct) -> Result<()> {
        for singleton in &self.singletons {
            Self::add_singleton(item, singleton.1)?;
        }

        Ok(())
    }
    fn add_singleton(item: &mut ItemStruct, singleton: &SingletonIdentifier) -> Result<()> {
        match &mut item.fields {
            Fields::Named(named) => {
                let mut singleton_field = Field {
                    attrs: Default::default(),
                    vis: Visibility::Inherited,
                    ident: Some(singleton.ident()),
                    colon_token: None,
                    ty: singleton.stored(),
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