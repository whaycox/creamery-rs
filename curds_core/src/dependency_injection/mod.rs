use super::*;

mod maps_transient;
mod injects_dependencies;
mod forwards_providers;
mod clones_providers;
mod generates_transient;
mod generates_singleton;

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