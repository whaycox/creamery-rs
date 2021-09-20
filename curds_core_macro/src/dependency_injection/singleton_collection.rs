use super::*;

pub struct SingletonCollection {
    singletons: HashSet<String>,
    types: HashMap<Type, SingletonIdentifier>,
    providers: HashMap<Ident, SingletonIdentifier>,
}

impl SingletonCollection {
    pub fn new () -> Self {
        Self {
            singletons: HashSet::new(),
            types: HashMap::new(),
            providers: HashMap::new(),
        }
    }

    fn register_singleton(&mut self, identifier: SingletonIdentifier) -> Option<SingletonIdentifier> {
        let singleton = identifier
            .ident()
            .to_string();
        if self.singletons.contains(&singleton) {
            let mut replacement = SingletonIdentifier::new();
            while self.singletons.contains(&replacement.ident().to_string()) {
                replacement = SingletonIdentifier::new()
            }
            self.singletons.insert(replacement.ident().to_string());
            Some(replacement)
        }
        else { 
            self.singletons.insert(singleton);
            None 
        }
    }

    pub fn register_type(&mut self, singleton: Type, identifier: SingletonIdentifier) -> Option<SingletonIdentifier> {
        match self.types.get(&singleton) {
            Some(stored) => Some(stored.clone()),
            None => match self.register_singleton(identifier.clone()) {
                None => {
                    self.types.insert(singleton, identifier);
                    None
                },
                Some(replaced) => {
                    self.types.insert(singleton, replaced.clone());
                    Some(replaced)
                }
            }
        }
    }

    pub fn register_provider(&mut self, provider: Ident, identifier: SingletonIdentifier) -> Option<SingletonIdentifier> {
        match self.providers.get(&provider) {
            Some(stored) => Some(stored.clone()),
            None => match self.register_singleton(identifier.clone()) {
                None => {
                    self.providers.insert(provider, identifier);
                    None
                },
                Some(replaced) => {
                    self.providers.insert(provider, replaced.clone());
                    Some(replaced)
                }
            }
        }
    }
}