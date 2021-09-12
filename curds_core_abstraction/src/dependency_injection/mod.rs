use super::*;

pub trait ServiceGenerator<TService: 'static> {
    fn generate(&self) -> TService;
}
impl<TProvider, TService: 'static> ServiceGenerator<TService> for Rc<TProvider>
where TProvider : ServiceGenerator<TService>  {
    fn generate(&self) -> TService { <TProvider as ServiceGenerator<TService>>::generate(&*self)  }
}

pub trait Injected<TProvider> {
    fn inject(provider: &TProvider) -> Rc<Self>;
}

pub trait Scoped {
    fn scope(&self) -> Self;
}
impl<TProvider> Scoped for Rc<TProvider>
where TProvider : Scoped {
    fn scope(&self) -> Self { Rc::new(<TProvider as Scoped>::scope(&*self)) }
}