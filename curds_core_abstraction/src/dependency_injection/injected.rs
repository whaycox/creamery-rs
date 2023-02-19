pub trait Injected<'provider, TProvider> {
    fn inject(provider: &'provider mut TProvider) -> Self;
}