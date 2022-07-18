use std::rc::Rc;

pub trait ServiceGenerator<TService: 'static> {
    fn generate(&self) -> TService;
}
impl<TProvider, TService: 'static> ServiceGenerator<Rc<TService>> for TProvider
where TProvider: ServiceGenerator<TService> {
    fn generate(&self) -> Rc<TService> {
        Rc::new(<TProvider as ServiceGenerator<TService>>::generate(self))
    }
}