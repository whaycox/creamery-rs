pub trait ServiceGenerator<TService: 'static> {
    fn generate(&self) -> TService;
}