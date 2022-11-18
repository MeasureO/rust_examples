struct Sequence3<T> {
    first: T,
    second: T,
    third: T,
}

// Read this as: for any type `T`, implement for `Sequence<T>` ...
impl<T> Sequence3<T> {
    pub fn new(first: T, second: T, third: T) -> Self {
        Self { first, second, third }
    }
