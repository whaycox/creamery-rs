pub trait ServiceGenerator<TService> {
    fn generate(&self) -> TService;
}