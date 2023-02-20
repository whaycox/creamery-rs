pub trait Injected<TProvider> {
    fn inject(provider: &mut TProvider) -> Self;
}