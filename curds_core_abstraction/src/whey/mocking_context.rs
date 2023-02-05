pub trait MockingContext<TWheyMock> {
    fn mocked(&self) -> TWheyMock;
}