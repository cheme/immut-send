extern crate serde;

use super::*;

use ::arc::ArcRef;
use ::rc::RcRef;
use ::clone::CloneRef;
use self::serde::{Serializer,Serialize,Deserialize,Deserializer};
use std::marker::PhantomData;

#[derive(Debug)]
/// Tech struct for impl
pub struct SerRef<T, R>(pub R, PhantomData<T>);
#[cfg(feature = "with-serde")]
impl<T, R> SerRef<T, R> {
  #[inline]
  pub fn new(r: R) -> Self {
    SerRef(r, PhantomData)
  }
}

impl<T: Serialize, R: Ref<T>> Serialize for SerRef<T, R> {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    self.0.borrow().serialize(serializer)
  }
}

impl<'de, T: Deserialize<'de>, R: Ref<T>> Deserialize<'de> for SerRef<T, R> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let t = T::deserialize(deserializer)?;
    Ok(SerRef::new(<R as Ref<T>>::new(t)))
  }
}

impl<T: Serialize> Serialize for ArcRef<T> {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    let a: &T = self.borrow();
    a.serialize(serializer)
  }
}

impl<'de, T: Clone + Send + Deserialize<'de>> Deserialize<'de> for ArcRef<T> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let t: T = T::deserialize(deserializer)?;
    Ok(<Self as Ref<T>>::new(t))
  }
}

impl<T: Serialize> Serialize for RcRef<T> {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    let a: &T = self.borrow();
    a.serialize(serializer)
  }
}

impl<'de, T: Clone + Send + Deserialize<'de>> Deserialize<'de> for RcRef<T> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let t: T = T::deserialize(deserializer)?;
    Ok(<Self as Ref<T>>::new(t))
  }
}

impl<T: Serialize> Serialize for CloneRef<T> {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    let a: &T = self.borrow();
    a.serialize(serializer)
  }
}

impl<'de, T: Clone + Send + Deserialize<'de>> Deserialize<'de> for CloneRef<T> {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let t: T = T::deserialize(deserializer)?;
    Ok(<Self as Ref<T>>::new(t))
  }
}

