use super::*;

pub struct SingletonDescription {
    pub requested: Type,
    pub stored: Type,
    pub generation: TokenStream,
}