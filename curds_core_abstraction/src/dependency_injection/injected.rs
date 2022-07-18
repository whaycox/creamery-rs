pub trait Injected<TProvider> {
    fn inject(provider: &TProvider) -> Self;
}