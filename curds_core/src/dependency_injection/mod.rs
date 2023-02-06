mod generates_transient;
mod generates_singleton;
mod injects_dependencies;
mod clones;
mod scopes;
mod forwards_transient;
mod forwards_singleton;
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
        fn foo(&self) -> u32;
    }

    #[injected]
    pub struct ConcreteFoo {}
    impl Foo for ConcreteFoo {
        fn foo(&self) -> u32 { EXPECTED_FOO }
    }

    #[injected]
    pub struct IncrementingFoo {
        #[defaulted]
        value: Cell<u32>,
    }
    impl Foo for IncrementingFoo {
        fn foo(&self) -> u32 {
            let value = self.value.get();
            self.value.set(value + 1);
            value
        }
    }

    #[injected]
    pub struct SeededFoo {
        #[defaulted(Cell::new(EXPECTED_FOO))]
        seeded_value: Cell<u32>,
    }
    impl Foo for SeededFoo {
        fn foo(&self) -> u32 {
            let value = self.seeded_value.get();
            self.seeded_value.set(value + 1);
            value
        }
    }

    #[injected]
    pub struct BarredFoo {
        bar: Rc<dyn Bar>,
    }
    impl Foo for BarredFoo {
        fn foo(&self) -> u32 { EXPECTED_FOO * self.bar.bar() }
    }

    pub const EXPECTED_BAR: u32 = 72;
    pub trait Bar {
        fn bar(&self) -> u32;
    }

    #[injected]
    pub struct ConcreteBar {}
    impl Bar for ConcreteBar {
        fn bar(&self) -> u32 { EXPECTED_BAR }
    }

    #[injected]
    pub struct FooedBar {
        foo: Box<dyn Foo>,
    }
    impl Bar for FooedBar {
        fn bar(&self) -> u32 { EXPECTED_BAR * self.foo.foo() }
    }
}