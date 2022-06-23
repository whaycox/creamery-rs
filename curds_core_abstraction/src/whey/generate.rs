use super::*;

pub struct GenerateExpectation<TMocked> {
    remaining: Cell<Option<u32>>,
    generator: Box<dyn MockGenerate<TMocked>>,
}
impl<TMocked> GenerateExpectation<TMocked> {
    pub fn is_exhausted(&self) -> bool {
        match self.remaining.borrow().get() {
            Some(calls) => calls > 0,
            None => false,
        }
    }
    
    pub fn consume(&self) -> TMocked {
        let mocked = self.generator.generate();
        match self.remaining.borrow().get() {
            Some(calls) => self.remaining.set(Some(calls - 1)),
            None => {},
        }
        mocked
    }
}
impl<TMocked> Default for GenerateExpectation<TMocked>
where TMocked : Default {
    fn default() -> Self {
        Self { 
            remaining: Cell::new(None), 
            generator: Box::new(DefaultGenerator{}),
        }
    }
}

pub trait MockGenerate<TMocked> {
    fn generate(&self) -> TMocked;
}

pub struct DefaultGenerator {}
impl<TMocked> MockGenerate<TMocked> for DefaultGenerator
where TMocked : Default {
    fn generate(&self) -> TMocked { Default::default() }
}

pub struct SomeGenerator<TMocked> {
    delegate: Box<dyn Fn() -> TMocked>,
}
impl<TMocked> SomeGenerator<TMocked> {
    pub fn new(delegate: Box<dyn Fn() -> TMocked>) -> Self {
        Self { 
            delegate: delegate, 
        }
    }
}
impl<TMocked> MockGenerate<TMocked> for SomeGenerator<TMocked> {
    fn generate(&self) -> TMocked { (self.delegate)() }
}