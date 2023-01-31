#[cfg(test)]
mod tests {
    use super::super::*;

    #[whey_mock]
    trait Foo {
        fn shared_foo(&self);
        fn exclusive_foo(&mut self);
    }

    #[test]
    fn verifies_shared_counts() {
        for count in 1..10 {
            verifies_shared_counts_helper(count);
        }
        
    }
    fn verifies_shared_counts_helper(count: u32) {
        let foo = WheyFoo::construct();

        for _ in 0..count {
            foo.shared_foo();
        }

        foo.assert_shared_foo(count);
    }

    #[test]
    fn verifies_exclusive_counts() {
        for count in 1..10 {
            verifies_exclusive_counts_helper(count);
        }
        
    }
    fn verifies_exclusive_counts_helper(count: u32) {
        let mut foo = WheyFoo::construct();

        for _ in 0..count {
            foo.exclusive_foo();
        }

        foo.assert_exclusive_foo(count);
    }
}