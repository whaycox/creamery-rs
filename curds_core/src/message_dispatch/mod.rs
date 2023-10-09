mod basic_message;
mod basic_request;
mod complex_message;
mod complex_request;
mod chain_message;
mod chain_request;
//mod default_pipeline_message;
//mod default_pipeline_request;
//mod default_chain;
//mod generic_dispatch;

#[cfg(test)]
use super::*;

#[cfg(test)]
pub use simple::*;

#[cfg(test)]
mod simple {
    use super::*;

    pub const EXPECTED_FOO: u32 = 876;

    #[derive(Debug)]
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
    impl PartialEq for FooMessage {
        fn eq(&self, other: &Self) -> bool {
            self.foo == other.foo
        }
    }

    #[injected]
    pub struct FooMessageContext {}

    #[injected]
    pub struct FooRepositoryContext {
        pub repo: Rc<dyn FooRepository>,
    }

    pub trait FooRepository {
        fn store(&self, foo: u32);
        fn get(&self) -> Option<u32>;
    }
    #[injected]
    pub struct ConcreteRepository {
        #[defaulted]
        repo: Cell<Option<u32>>,
    }
    impl FooRepository for ConcreteRepository {
        fn store(&self, foo: u32) {
            self.repo.set(Some(foo))
        }
        fn get(&self) -> Option<u32> {
            self.repo.get()
        }
    }

    #[derive(Debug)]
    pub struct FooMessageError {}
    impl Display for FooMessageError {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "FooMessageError")
        }
    }
    impl Error for FooMessageError {}
}