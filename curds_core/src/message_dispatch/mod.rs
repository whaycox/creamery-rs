use super::*;

mod basic_message;

#[cfg(test)]
pub use simple::*;

#[cfg(test)]
mod simple {
    use super::*;

    pub const EXPECTED_FOO: u32 = 876;

    pub struct FooMessage {
        pub foo: u32,
    } 
    impl FooMessage {
        pub fn new() -> Self {
            Self {
                foo: EXPECTED_FOO,
            }
        }
    }

    #[injected]
    pub struct FooMessageContext {}
}