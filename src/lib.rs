use std::borrow::Borrow;

pub mod arc;
pub mod rc;
pub mod clone;
#[cfg(feature = "with-serde")]
pub mod ser;

/// Type with an associated type being Send and which is possible to switch to its original type
/// Copy of content may be involved in the precess.
pub trait SRef: Sized {
  type Send: SToRef<Self>;
  fn get_sendable(self) -> Self::Send;
}
pub trait SToRef<T: SRef>: Send + Sized {
  //  type Ref : Ref<T,Send=Self>;
  fn to_ref(self) -> T;
}

/// trait to allow variant of Reference in mydht. Most of the time if threads are involved (depends on
/// Spawner used) and Peer struct is big enough we use Arc.
/// Note that Ref is semantically wrong it should be val. The ref here expect inner immutability.
///
/// Principal use case is using Rc which is not sendable.
/// TODO name should change to Immut
/// TODO Borrow switch to Deref
pub trait Ref<T>: SRef + Borrow<T> {
  //type Ref<'a,T>;
  fn new(t: T) -> Self;
  // only possible with associated lifetime, for now borrow is fine if we do not compose enum : see
  // numerous clone of voting machine type https://github.com/rust-lang/rust/issues/44265
  //fn get_ref(t) -> Self::Ref;
}


#[macro_export]
macro_rules! sref_self(($ty:ident) => (

  impl SRef for $ty {
    type Send = $ty;
    #[inline]
    fn get_sendable(self) -> Self::Send {
      self
    }
  }
  impl SToRef<$ty> for $ty {
    #[inline]
    fn to_ref(self) -> $ty {
      self
    }
  }

));

