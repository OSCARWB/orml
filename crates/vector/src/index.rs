//! This module contains implementations for indexing operations
//! on the elements of the Vector

use std::ops::{Index, IndexMut};

use super::Vector;

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
	use crate::tests::{VUsizeN, SIZE};

	#[test]
	fn valid_index() {
		let mut vec: VUsizeN = Vector::new();
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
		let mut vec: VUsizeN = Vector::new();
		for i in 0..SIZE {
			vec[4] = i;
		}

		for i in 0..SIZE {
			assert_eq!(vec[4], i);
		}
	}
}
