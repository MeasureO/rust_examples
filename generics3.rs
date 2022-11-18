use std::ops::Add;

impl<T> Sequence3<T> where T: Add<Output = T> {
    pub fn sum(&self) -> T {
        self.first + self.second + self.third
    }
}
