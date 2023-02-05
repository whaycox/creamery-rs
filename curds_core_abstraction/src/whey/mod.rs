mod setup;
mod generate;
mod compare;
mod setup_error;

pub use setup::*;
pub use compare::*;
pub use setup_error::*;
pub use generate::*;

use std::cell::Cell;
use std::borrow::Borrow;

pub trait Whey {
    fn reset(&self);
}

pub trait WheyContext<TWheyMock> {
    fn mocked(&self) -> TWheyMock;
}