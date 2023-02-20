pub trait ServiceGenerator<TService: 'static> {
    fn generate(&mut self) -> TService;
}