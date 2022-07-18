pub trait Scoped {
    fn scope(&self) -> Self;
}