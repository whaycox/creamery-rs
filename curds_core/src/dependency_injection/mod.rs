mod generates_transient;
mod generates_singleton;
mod injects_dependencies;
mod clones;
mod forwards_transient;
mod forwards_singleton;
mod forwards_singleton_promoted;
mod scopes;
mod generic_service;

#[cfg(test)]
use super::*;

#[cfg(test)]
pub use simple::*;

#[cfg(test)]
mod simple {
    use super::*;

    pub const EXPECTED_FOO: u32 = 123;

    pub trait Foo {
        fn foo(&mut self) -> u32;
    }

    #[injected]
    pub struct ConcreteFoo {}
    impl Foo for ConcreteFoo {
        fn foo(&mut self) -> u32 { EXPECTED_FOO }
    }
 
    #[injected]
    #[derive(Clone)]
    pub struct IncrementingFoo {
        #[defaulted]
        value: u32,
    }
    impl Foo for IncrementingFoo {
        fn foo(&mut self) -> u32 {
            let value = self.value;
            self.value += 1;
            value
        }
    }

    #[injected]
    pub struct SeededFoo {
        #[defaulted(EXPECTED_FOO)]
        seeded_value: u32,
    }
    impl Foo for SeededFoo {
        fn foo(&mut self) -> u32 {
            let value = self.seeded_value;
            self.seeded_value += 1;
            value
        }
    }

    #[injected]
    pub struct BarredFoo {
        bar: Singleton<Box<dyn Bar>>,
    }
    impl Foo for BarredFoo {
        fn foo(&mut self) -> u32 { EXPECTED_FOO * self.bar.write().unwrap().bar() }
    }

    pub const EXPECTED_BAR: u32 = 72;
    pub trait Bar {
        fn bar(&mut self) -> u32;
    }

    #[injected]
    pub struct ConcreteBar {}
    impl Bar for ConcreteBar {
        fn bar(&mut self) -> u32 { EXPECTED_BAR }
    }

    #[injected]
    pub struct FooedBar {
        foo: Box<dyn Foo>,
    }
    impl Bar for FooedBar {
        fn bar(&mut self) -> u32 { EXPECTED_BAR * self.foo.foo() }
    }
}