use super::*;
use std::rc::Rc;

/// Rc is used locally (the content size is not meaningless), a copy of the content is done if
/// threads are used.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RcRef<T>(Rc<T>);

#[derive(Clone, Eq, PartialEq)]
pub struct ToRcRef<T>(T);

impl<T> Borrow<T> for RcRef<T> {
  #[inline]
  fn borrow(&self) -> &T {
    self.0.borrow()
  }
}

impl<T: Send + Clone> SRef for RcRef<T> {
  type Send = ToRcRef<T>;
  #[inline]
  fn get_sendable(self) -> Self::Send {
    match Rc::try_unwrap(self.0) {
      Ok(content) => ToRcRef(content),
      Err(rcref) => ToRcRef((&*rcref).clone()),
    }
  }
}
impl<T: Send + Clone> Ref<T> for RcRef<T> {
  #[inline]
  fn new(t: T) -> Self {
    RcRef(Rc::new(t))
  }
}

impl<T: Send + Clone> SToRef<RcRef<T>> for ToRcRef<T> {
  #[inline]
  fn to_ref(self) -> RcRef<T> {
    RcRef(Rc::new(self.0))
  }
}

impl<T: Send + Clone> Borrow<T> for ToRcRef<T> {
  #[inline]
  fn borrow(&self) -> &T {
    self.0.borrow()
  }
}

