use super::*;

mod maps_transient;
mod injects_dependencies;
mod nests_providers;


#[cfg(test)]
trait Foo {
    fn foo(&self) -> u32;
}

#[cfg(test)]
const EXPECTED_FOO: u32 = 123;

#[cfg(test)]
trait Bar {
    fn bar(&self) -> u32;
}

#[cfg(test)]
const EXPECTED_BAR: u32 = 72;