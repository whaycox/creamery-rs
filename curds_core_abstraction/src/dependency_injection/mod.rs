use super::*;

pub trait ServiceGenerator<TService: 'static> {
    fn generate(&self) -> TService;
}

pub trait Injected<TProvider> {
    fn inject(provider: &TProvider) -> Self;
}

pub trait RootInjected {
    fn root_inject() -> Self;
}