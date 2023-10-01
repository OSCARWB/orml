#![warn(missing_docs)]
#![allow(dead_code)]

//! Mathimatical Vectors
//!
//! This module contains a generic Vector (Vector<T, const DIMS: usize>) that can be any type or size
//! and type defs of commonly used vector i.e Vec3f64

use std::fmt::{Debug, Display};

pub mod arithmetic;
pub mod index;
pub mod iter;
pub mod ordering;
pub mod typedefs;

/// A Mathimatical Vector
/// Takes in a type T as the underlying type and DIMS and the number of dimensions of the vector
#[derive(Debug)]
pub struct Vector<T, const DIMS: usize> {
	vals: [T; DIMS],
}

impl<T, const DIMS: usize> Vector<T, DIMS> {
	/// Creates a new Vector<T,DIMS> from an Array [T;DIMS]
	#[inline]
	pub fn from_array(arr: [T; DIMS]) -> Self {
		arr.into()
	}

	/// Returns an Array [T;DIMS] from the interal representation
	#[inline]
	pub fn to_array(self) -> [T; DIMS] {
		self.vals
	}

	/// Returns the numer of dimensions of the Vector
	// Used len() to keep consistent with other Rust containers
	#[inline]
	pub fn len(&self) -> usize {
		DIMS
	}
	/// Returns true if the Vector is 0 dimensional
	#[inline]
	pub fn is_empty(&self) -> bool {
		DIMS == 0
	}
}

#[allow(clippy::uninit_assumed_init)]
impl<T, const DIMS: usize> Vector<T, DIMS>
where
	T: Default,
{
	/// Create a new Vector
	#[inline]
	pub fn new() -> Self {
		Self {
			vals: Self::new_arr(),
		}
	}

	#[inline]
	fn new_arr() -> [T; DIMS] {
		// println!("jere");
		// let mut arr = unsafe {
		// 	std::mem::MaybeUninit::<[T; DIMS]>::uninit()
		// 		.assume_init()
		// };

		// for i in 0..DIMS {
		// 	println!("jere2");
		// 	arr[i] = Default::default();
		// 	println!("jere3");
		// }

		// arr

		let mut vec = Vec::with_capacity(DIMS);
		for _ in 0..DIMS {
			vec.push(Default::default());
		}

		unsafe { vec.try_into().unwrap_unchecked() }
	}
}

impl<T, const DIMS: usize> Default for Vector<T, DIMS>
where
	T: Default + Debug,
{
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl<T, const DIMS: usize> From<[T; DIMS]> for Vector<T, DIMS> {
	#[inline]
	fn from(value: [T; DIMS]) -> Self {
		Self { vals: value }
	}
}

impl<T, const DIMS: usize> Clone for Vector<T, DIMS>
where
	T: Clone,
{
	#[inline]
	fn clone(&self) -> Self {
		Self {
			vals: self.vals.clone(),
		}
	}
}

impl<T, const DIMS: usize> Copy for Vector<T, DIMS> where T: Copy {}

impl<T, const DIMS: usize> Display for Vector<T, DIMS>
where
	T: Display,
{
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[")?;
		for e in &self.vals {
			write!(f, " {},", e)?;
		}
		write!(f, " ]")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		let i1: Vector<i32, 64> = Vector::new();
		let i2 = Vector::from_array([Default::default(); 64]);
		assert_eq!(i1, i2);

		let f1: Vector<f64, 64> = Vector::new();
		let f2 = Vector::from_array([Default::default(); 64]);
		assert_eq!(f1, f2);
	}

	#[test]
	fn clone() {
		let a = Vector::from_array([1, 1, 1]);

		assert_eq!(a, a.clone());
	}
}
