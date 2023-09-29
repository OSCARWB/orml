//! This module contains implementations for indexing operations
//! on the elements of the Vector

use std::ops::{Index, IndexMut};

use super::Vector;

impl<T, const DIMS: usize> Index<usize> for Vector<T, DIMS> {
	type Output = T;

	#[inline]
	fn index(&self, index: usize) -> &Self::Output {
		if index < DIMS {
			&self.vals[index]
		} else {
			panic!()
		}
	}
}

// impl<T, const DIMS: usize> Index<&[usize]> for Vector<T, DIMS> {
// 	type Output = Vector;
// 	fn index(&self, index: &[usize]) -> &Self::Output {
// 		if index < DIMS {
// 			&self.vals[index]
// 		} else {
// 			panic!()
// 		}
// 	}
// }

impl<T, const DIMS: usize> IndexMut<usize> for Vector<T, DIMS> {
	#[inline]
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
	use crate::typedefs::Vec3i32;

	use super::*;

	#[test]
	fn valid_index() {
		let mut vec: Vec3i32 = Vector::new();
		for i in 0..3 {
			vec[i] = i as i32;
		}

		for i in 0..3 {
			assert_eq!(vec[i], i as i32);
		}
	}

	#[test]
	#[should_panic]
	fn invalid_index() {
		let mut vec: Vec3i32 = Vector::new();
		for i in 0..3 {
			vec[4] = i;
		}

		for i in 0..3 {
			assert_eq!(vec[4], i);
		}
	}

	#[test]
	fn mutation() {
		let mut vec: Vec3i32 = Vector::new();
		let expected: Vec3i32 = [0, 5, 0].into();

		for i in 0..3 {
			vec[i] = i as i32;
		}

		vec[1] = 5;
		vec[2] = 0;

		for i in 0..3 {
			assert_eq!(vec[i], expected[i]);
		}
	}
}
