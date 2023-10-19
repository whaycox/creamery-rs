use super::*;

#[whey_mock]
pub trait Terminal {
    fn write(&self, message: &str);
}