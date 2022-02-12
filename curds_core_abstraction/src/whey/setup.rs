use super::*;

pub trait Setup<TInput, TMocked> {
    fn is_exhausted(&self) -> bool;
    fn set_times(&self, times: u32);
    fn consume(&self, input: TInput) -> Result<TMocked, SetupError>;
}

pub struct ValueSetup<TInput, TMocked>
where TMocked : Copy {
    input_comparison: Box<dyn InputCompare<TInput>>,
    mocked: TMocked,
    times: Cell<u32>,
}
impl<TInput, TMocked> ValueSetup<TInput, TMocked>
where TMocked : Copy {
    pub fn new(input_comparison: Box<dyn InputCompare<TInput>>, mocked: TMocked) -> Self {
        Self {
            input_comparison: input_comparison,
            mocked: mocked,
            times: Cell::new(1),
        }
    }
}
impl<TInput, TMocked> Setup<TInput, TMocked> for ValueSetup<TInput, TMocked>
where TMocked : Copy {    
    fn is_exhausted(&self) -> bool { self.times.get() == 0 }

    fn set_times(&self, times: u32) { self.times.set(times) }

    fn consume(&self, input: TInput) -> Result<TMocked, SetupError> {
        if self.is_exhausted() {
            Err(SetupError::ExhaustedConsumption)
        }
        else if !self.input_comparison.is_expected(input) {
            Err(SetupError::InputComparison)
        }
        else {
            self.times.set(self.times.get() - 1);
            Ok(self.mocked)
        }
    }
}
impl<TInput, TMocked> Drop for ValueSetup<TInput, TMocked>
where TMocked : Copy {
    fn drop(&mut self) {
        if !self.is_exhausted() {
            panic!("setup must be exhausted prior to dropping")
        }
    }
}