pub trait ServiceGenerator<TService: 'static> {
    fn generate(&mut self) -> TService;
}
pub trait ServiceLender<TService: 'static> {
    fn lend(&mut self) -> &TService;
    fn lend_mut(&mut self) -> &mut TService;
}