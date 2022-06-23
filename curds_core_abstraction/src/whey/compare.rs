use super::*;

pub struct CompareExpectation<TInput> {
    remaining: Cell<Option<u32>>,
    comparison: Box<dyn InputCompare<TInput>>,
}
impl<TInput> CompareExpectation<TInput> {
    pub fn is_exhausted(&self) -> bool {
        match self.remaining.borrow().get() {
            Some(calls) => calls > 0,
            None => false,
        }
    }

    pub fn consume(&self, input: TInput) -> bool {
        let comparison = self.comparison.is_expected(input);
        match self.remaining.borrow().get() {
            Some(calls) => self.remaining.set(Some(calls - 1)),
            None => {},
        }
        comparison
    }
}
impl<TInput> Default for CompareExpectation<TInput> {
    fn default() -> Self {
        Self { 
            remaining: Cell::new(None), 
            comparison: Box::new(AnyCompare{}),
        }
    }
} 

pub trait InputCompare<TInput> {
    fn is_expected(&self, input: TInput) -> bool;
}

pub struct AnyCompare {}
impl AnyCompare {
    pub fn new() -> Self { Self {} }
}
impl<TInput> InputCompare<TInput> for AnyCompare {
    fn is_expected(&self, input: TInput) -> bool { true }
}

pub struct SomeCompare {}
impl SomeCompare {
    pub fn new() -> Self { Self {} }
}
impl<TInput> InputCompare<Option<TInput>> for SomeCompare {
    fn is_expected(&self, input: Option<TInput>) -> bool { input.is_some() }
}

pub struct NoneCompare {}
impl NoneCompare {
    pub fn new() -> Self { Self {} }
}
impl<TInput> InputCompare<Option<TInput>> for NoneCompare {
    fn is_expected(&self, input: Option<TInput>) -> bool { input.is_none() }
}

pub struct EqualityCompare<TInput>
where TInput : PartialEq {
    expected: TInput,
}
impl<TInput> InputCompare<TInput> for EqualityCompare<TInput>
where TInput : PartialEq {
    fn is_expected(&self, input: TInput) -> bool { PartialEq::eq(&self.expected, &input) }
}