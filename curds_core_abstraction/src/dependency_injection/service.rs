use std::rc::Rc;
use std::sync::RwLock;

pub trait ServiceGenerator<TService> {
    fn generate(&self) -> TService;
}

pub type Singleton<TService> = Rc<RwLock<TService>>;