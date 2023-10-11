#[cfg(test)]
mod tests {
    use super::super::*;

    #[message_dispatch(TestMessages ! FooMessageError)]
    #[first(FooMessage ~ FooMessageContext)]
    struct TestMessagesProvider<T> {
        #[defaulted]
        phantom: PhantomData<T>
    }

    impl FirstHandler for FooMessageContext {
        fn handle(&self, _: &dyn TestMessages, _: FooMessage) -> Result<(), FooMessageError> {
            Ok(())
        }
    }

    #[whey_context(TestMessagesProvider<u32>)]
    struct GenericDispatchContext {}

    #[whey(GenericDispatchContext ~ context)]
    fn handles_first() {
        context
            .test_type()
            .first(FooMessage::new())
            .unwrap()
    }
}