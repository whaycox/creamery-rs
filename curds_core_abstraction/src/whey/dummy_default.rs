pub trait DummyDefault {
    fn dummy() -> Self;
}

impl<TDummy: Default> DummyDefault for TDummy {
    fn dummy() -> Self { Default::default() }
}