#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use super::super::*;

    #[message_dispatch(TestMessages)]
    #[first(FooMessage ~ FooMessageContext)]
    struct TestMessagesProvider<T> {
        #[defaulted]
        phantom: PhantomData<T>
    }

    impl FirstHandler for FooMessageContext {
        fn handle(&self, _dispatch: &dyn TestMessages, input: &FooMessage) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn handles_first() {
        let provider = TestMessagesProvider::<u32>::construct();
        provider.first(FooMessage::new()).unwrap()
    }
}