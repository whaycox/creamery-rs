mod basic_message;
mod basic_request;
mod complex_message;
mod complex_request;

#[cfg(test)]
use super::*;

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

        pub fn test(value: u32) -> Self {
            Self {
                foo: value,
            }
        }
    }

    #[injected]
    pub struct FooMessageContext {}

    #[derive(Debug)]
    pub struct FooMessageError {
        message: String,
    }

    impl FooMessageError {
        pub fn new(message: &str) -> Self {
            Self {
                message: message.to_owned()
            }
        }
    }
    impl Display for FooMessageError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.message)
        }
    }
    impl Error for FooMessageError {}
}