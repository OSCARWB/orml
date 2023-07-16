//! Mathimatical Vectors
//!
//! This module contains a generic Vector (Vector<T, const DIMS: usize>) that can be any type or size
//! and type defs of commonly used vector i.e Vec3f64

#![allow(dead_code)]

use std::{ops::{Add, Mul}, array};

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

	pub fn iter(&self) -> std::slice::Iter<T> {
		self.vals.iter()
	}

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

impl<T, const DIMS: usize> Vector<T, DIMS>
where
	T: Add<Output = T> + Copy + Default + Mul<Output = T>,
{
	pub fn length_squared(&self) -> T {
		self.vals
			.into_iter()
			.fold(Default::default(), |acc, x| (x * x) + acc)
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

    type IntoIter = array::IntoIter<T,DIMS>;

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

	#[test]
	fn length_squared() {
		let expected1: VUsizeN = [1, 1, 1].into();
		assert_eq!(expected1.length_squared(), 3);

		let expected2: VUsizeN = [2, 2, 2].into();
		assert_eq!(expected2.length_squared(), 12);
	}
}
