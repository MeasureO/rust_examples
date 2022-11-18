use std::cmp::PartialEq;

// For all types T implementing PartialEq, implement for Sequence3<T> ...
impl<T: PartialEq> Sequence3<T> {
    pub fn all_same(&self) -> bool {
        self.first == self.second && self.second == self.third
    }
}
  
use std::cmp::PartialEq;

// We can also move the type bound to a `where` clause.
impl<T> Sequence3<T> where T: PartialEq {
    pub fn all_same(&self) -> bool {
        self.first == self.second && self.second == self.third
    }
}
