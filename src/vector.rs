//! Mathimatical Vectors
//!
//! This module contains a generic Vector (Vector<T, const DIMS: usize>) that can be any type or size
//! and type defs of commonly used vector i.e Vec3f64

use std::ops::{Index, IndexMut};

/// A Mathimatical Vector
/// Takes in a type T as the underlying value and DIMS and the number of dimensions of the vector
pub struct Vector<T, const DIMS: usize> {
	vals: [T; DIMS],
}

impl<T: Copy + Default, const DIMS: usize> Vector<T, DIMS> {
	/// Create a new Vector
	pub fn new() -> Self {
		Self {
			vals: [Default::default(); DIMS],
		}
	}
}

impl<T: Copy + Default, const DIMS: usize> Default for Vector<T, DIMS> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T, const DIMS: usize> Index<usize> for Vector<T, DIMS> {
	type Output = T;
	fn index(&self, index: usize) -> &Self::Output {
		if index < DIMS {
			&self.vals[index]
		} else {
			panic!()
		}
	}
}

impl<T, const DIMS: usize> Index<&[usize]> for Vector<T, DIMS> {
	type Output = Vector;
	fn index(&self, index: &[usize]) -> &Self::Output {
		if index < DIMS {
			&self.vals[index]
		} else {
			panic!()
		}
	}
}

impl<T, const DIMS: usize> IndexMut<usize> for Vector<T, DIMS> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		if index < DIMS {
			&mut self.vals[index]
		} else {
			panic!()
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn valid_index() {
		const SIZE: usize = 3;
		let mut vec: Vector<usize,SIZE> = Vector::new();
		for i in 0..SIZE {
			vec[i] = i;
		}

		for i in 0..SIZE {
			assert_eq!(vec[i], i);
		}
	}

	#[test]
	#[should_panic]
	fn invalid_index() {
		const SIZE: usize = 3;
		let mut vec: Vector<usize,SIZE> = Vector::new();
		for i in 0..SIZE {
			vec[4] = i;
		}

		for i in 0..SIZE {
			assert_eq!(vec[4], i);
		}
	}
}
