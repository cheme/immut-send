use super::*;

use std::sync::Arc;

/// Arc is used to share peer or key val between threads
/// useless if no threads in spawners.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ArcRef<T>(Arc<T>);

/// ArcRef as any Ref is seen as read only, quite unsafe
unsafe impl<T> Sync for ArcRef<T> {}
/// ArcRef as any Ref is seen as read only, quite unsafe
unsafe impl<T: Send> Send for ArcRef<T> {}

impl<T> Borrow<T> for ArcRef<T> {
  #[inline]
  fn borrow(&self) -> &T {
    self.0.borrow()
  }
}

impl<T: Clone + Send> SRef for ArcRef<T> {
  type Send = ArcRef<T>;
  #[inline]
  fn get_sendable(self) -> Self::Send {
    self
  }
}
impl<T: Clone + Send> Ref<T> for ArcRef<T> {
  #[inline]
  fn new(t: T) -> Self {
    ArcRef(Arc::new(t))
  }
}
impl<T: Clone + Send> SToRef<ArcRef<T>> for ArcRef<T> {
  #[inline]
  fn to_ref(self) -> ArcRef<T> {
    self
  }
}


