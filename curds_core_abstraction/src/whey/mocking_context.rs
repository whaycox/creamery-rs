pub trait MockingContext<TWheyMock> {
    fn mocked(&mut self) -> TWheyMock;
}