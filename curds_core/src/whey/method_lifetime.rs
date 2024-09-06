#[cfg(test)]
mod tests {
    use super::super::*;
    use std::pin::Pin;
    use std::future::Future;

    #[whey_mock]
    trait LifetimeFoo {
        fn method_lifetime<'a>(&self, input: &'a CustomStruct) -> &'a str;
        fn future_method_lifetime<'a>(&self, input: &'a CustomStruct) -> Pin<Box<dyn Future<Output = u32> + Send + Sync + 'a>>;
    }

    #[test]
    fn can_handle_lifetime_methods() {
        let input = CustomStruct::default();
        let test_object = TestingLifetimeFoo::new();
        test_object.default_return_method_lifetime(|_| "can_handle_lifetime_methods");
        
        assert_eq!("can_handle_lifetime_methods", test_object.method_lifetime(&input));
    }

    #[tokio::test]
    async fn can_handle_nested_lifetime_methods() {
        let input = CustomStruct::default();
        let test_object = TestingLifetimeFoo::new();
        test_object.default_return_future_method_lifetime(|_| Box::pin(async { EXPECTED_INT }));
        
        assert_eq!(EXPECTED_INT, test_object.future_method_lifetime(&input).await);
    }
}