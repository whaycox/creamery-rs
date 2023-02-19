use super::*;

pub const SINGLETON_FIELD_PREFIX: &str = "_curds_core_singleton_";

#[derive(Clone)]
pub struct SingletonIdentifier {
    ident: String,
    stored: Type,
    generation: TokenStream,
}

impl SingletonIdentifier {
    pub fn new(description: &SingletonDescription) -> Self {
        let random_bytes = rand::thread_rng().gen::<[u8; 4]>();
        let mut singleton_identifier = String::new();
        for byte in random_bytes {
            singleton_identifier.push_str(&format!("{:X}", byte));
        }

        Self {
            ident: singleton_identifier,
            stored: description.stored.clone(),
            generation: description.generation.clone(),
        }
    }

    pub fn stored(&self) -> Type { self.stored.clone() }

    pub fn ident(&self) -> Ident {
        Ident::new(&format!("{}{}", SINGLETON_FIELD_PREFIX, self.ident), Span::call_site())
    }

    pub fn quote_initializer(&self) -> TokenStream {
        let ident = self.ident();
        let generation = &self.generation;

        quote! {
            constructed.#ident = Some(#generation);
        }
    }
    pub fn quote_initializer_attribute(&self) -> TokenStream {
        let singleton_initializer = self.quote_initializer();

        quote! {
            #[initializer(#singleton_initializer)]
        }
    }
}