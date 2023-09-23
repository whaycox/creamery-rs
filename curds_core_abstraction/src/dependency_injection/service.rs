pub trait ServiceGenerator<TService> {
    fn generate(&mut self) -> TService;
}