#![warn(missing_docs)]
#![allow(dead_code)]

//! Mathimatical Vectors
//!
//! This module contains a generic Vector (Vector<T, const DIMS: usize>) that can be any type or size
//! and type defs of commonly used vector i.e Vec3f64

use std::array;

pub mod arithmetic;
pub mod index;
pub mod ordering;
pub mod typedefs;

/// A Mathimatical Vector
/// Takes in a type T as the underlying value and DIMS and the number of dimensions of the vector
#[derive(Debug)]
pub struct Vector<T, const DIMS: usize> {
	vals: [T; DIMS],
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
	/// Returns an iter from the underlying array
	pub fn iter(&self) -> std::slice::Iter<T> {
		self.vals.iter()
	}
	/// Returns a mutable iter from the underlying array
	pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
		self.vals.iter_mut()
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

impl<T, const DIMS: usize> IntoIterator for Vector<T, DIMS> {
	type Item = T;

	type IntoIter = array::IntoIter<T, DIMS>;

	fn into_iter(self) -> Self::IntoIter {
		self.vals.into_iter()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	pub const SIZE: usize = 3;
	pub type VUsizeN = Vector<usize, SIZE>;
	pub type VIsizeN = Vector<isize, SIZE>;
	pub type VBoolN = Vector<bool, SIZE>;

	#[test]
	fn mutation() {
		let mut vec: VUsizeN = Vector::new();
		let expected: VUsizeN = [0, 5, 0].into();

		for i in 0..SIZE {
			vec[i] = i;
		}

		vec[1] = 5;
		vec[2] = 0;

		for i in 0..SIZE {
			assert_eq!(vec[i], expected[i]);
		}
	}
}
