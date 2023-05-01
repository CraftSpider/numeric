
pub trait IntoOwned<T> {
    fn into_owned(self) -> T;
}

impl<T> IntoOwned<T> for T {
    fn into_owned(self) -> T {
        self
    }
}

impl<T> IntoOwned<Vec<T>> for &[T]
where
    T: Clone,
{
    fn into_owned(self) -> Vec<T> {
        self.to_owned()
    }
}

impl<T> IntoOwned<Box<[T]>> for &[T]
where
    T: Copy,
{
    fn into_owned(self) -> Box<[T]> {
        self.into()
    }
}

impl<T> IntoOwned<Box<[T]>> for Vec<T> {
    fn into_owned(self) -> Box<[T]> {
        self.into_boxed_slice()
    }
}
