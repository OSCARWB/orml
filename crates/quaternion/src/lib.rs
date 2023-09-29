#![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
#![allow(dead_code)]

//! Quaternions
//!
//! This module contains a generic Quaternion that can be any type or size

use orml_vector::Vector;

pub mod arithmetic;
#[cfg(feature = "vector")]
pub mod vector;

pub mod ordering;

/// A Quaternion
/// Takes in a type T as the underlying type of the Quaternion

#[derive(Debug)]
pub struct Quaternion<T> {
	/// X component
	pub x: T,
	/// Y component
	pub y: T,
	/// Z component
	pub z: T,
	/// W component
	pub w: T,
}

impl<T> Quaternion<T> {
	/// TODO: Documentation
	pub fn zip(
		self,
		rhs: Self,
	) -> std::iter::Zip<std::array::IntoIter<T, 4>, std::array::IntoIter<T, 4>> {
		std::iter::zip::<[T; 4], [T; 4]>(self.into(), rhs.into())
	}

	/// Returns an Array [T;4] from the interal representation
	pub fn to_array(self) -> [T; 4] {
		self.into()
	}
}

impl<T> Quaternion<T>
where
	T: Copy,
{
	/// Creates a new Quaternion<T> from an Array [T;3]
	pub fn from_array(arr: [T; 4]) -> Quaternion<T> {
		arr.into()
	}
}

impl<T> From<[T; 4]> for Quaternion<T>
where
	T: Copy,
{
	fn from(value: [T; 4]) -> Self {
		Self {
			x: value[0],
			y: value[1],
			z: value[2],
			w: value[3],
		}
	}
}

impl<T> From<(T,T,T,T)> for Quaternion<T>
where
	T:,
{
	fn from(value: (T,T,T,T)) -> Self {
		Self {
			x: value.0,
			y: value.1,
			z: value.2,
			w: value.3,
		}
	}
}

impl<T> From<Quaternion<T>> for [T; 4] {
	fn from(value: Quaternion<T>) -> Self {
		[value.x, value.y, value.z, value.w]
	}
}

impl<T> Clone for Quaternion<T>
where
	T: Clone,
{
	fn clone(&self) -> Self {
		Self {
			x: self.x.clone(),
			y: self.y.clone(),
			z: self.z.clone(),
			w: self.w.clone(),
		}
	}
}

impl<T> Copy for Quaternion<T> where T: Copy {}
