#![warn(missing_docs)]
#![allow(dead_code)]

//! Mathimatical Vectors
//!
//! This module contains a generic Vector (Vector<T, const DIMS: usize>) that can be any type or size
//! and type defs of commonly used vector i.e Vec3f64

pub mod arithmetic;
pub mod index;
pub mod iter;
pub mod ordering;
pub mod typedefs;

/// A Mathimatical Vector
/// Takes in a type T as the underlying value and DIMS and the number of dimensions of the vector
#[derive(Debug)]
pub struct Vector<T, const DIMS: usize> {
	vals: [T; DIMS],
}

impl<T, const DIMS: usize> Vector<T, DIMS> {
	/// Creates a new Vector<T,DIMS> from an Array [T;DIMS]
	pub fn from_array(arr: [T; DIMS]) -> Self {
		arr.into()
	}
	/// Returns the numer of dimensions of the Vector
	// Used len() to keep consistent with other Rust containers
	pub fn len(&self) -> usize {
		DIMS
	}
	/// Returns true if the Vector is 0 dimensional
	pub fn is_empty(&self) -> bool {
		DIMS == 0
	}
}

impl<T, const DIMS: usize> Vector<T, DIMS>
where
	T: Copy + Default,
{
	/// Create a new Vector
	pub fn new() -> Self {
		Self {
			vals: [Default::default(); DIMS],
		}
	}
}

impl<T, const DIMS: usize> Default for Vector<T, DIMS>
where
	T: Copy + Default,
{
	fn default() -> Self {
		Self::new()
	}
}

impl<T, const DIMS: usize> From<[T; DIMS]> for Vector<T, DIMS> {
	fn from(value: [T; DIMS]) -> Self {
		Self { vals: value }
	}
}

impl<T, const DIMS: usize> Clone for Vector<T, DIMS>
where
	T: Clone,
{
	fn clone(&self) -> Self {
		Self {
			vals: self.vals.clone(),
		}
	}
}

impl<T, const DIMS: usize> Copy for Vector<T, DIMS> where T: Copy {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn clone() {
		let a = Vector::from_array([1, 1, 1]);

		assert_eq!(a, a.clone());
	}
}
