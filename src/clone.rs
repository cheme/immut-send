use super::*;



/// Content is already send and cloned as neededtsttststststs
/// location : only for small contents
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CloneRef<T>(T);

impl<T> Borrow<T> for CloneRef<T> {
  #[inline]
  fn borrow(&self) -> &T {
    &self.0
  }
}

impl<T: Send + Clone> SRef for CloneRef<T> {
  type Send = CloneRef<T>;
  #[inline]
  fn get_sendable(self) -> Self::Send {
    self
  }
}
impl<T: Send + Clone> Ref<T> for CloneRef<T> {
  #[inline]
  fn new(t: T) -> Self {
    CloneRef(t)
  }
}

impl<T: Send + Clone> SToRef<CloneRef<T>> for CloneRef<T> {
  #[inline]
  fn to_ref(self) -> CloneRef<T> {
    CloneRef(self.0)
  }
}

