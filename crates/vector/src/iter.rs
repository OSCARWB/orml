//! This module contains implementations for interators

use std::{array, slice};

use crate::Vector;

impl<T, const DIMS: usize> Vector<T, DIMS> {
	/// Returns an iter from the underlying array
	#[inline]
	pub fn iter(&self) -> std::slice::Iter<T> {
		self.vals.iter()
	}
	/// Returns a mutable iter from the underlying array
	#[inline]
	pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
		self.vals.iter_mut()
	}
}

impl<T, const DIMS: usize> IntoIterator for Vector<T, DIMS> {
	type Item = T;

	type IntoIter = array::IntoIter<T, DIMS>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.vals.into_iter()
	}
}

impl<'a, T, const DIMS: usize> IntoIterator for &'a Vector<T, DIMS> {
	type Item = &'a T;
	type IntoIter = slice::Iter<'a, T>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.vals.iter()
	}
}

impl<'a, T, const DIMS: usize> IntoIterator for &'a mut Vector<T, DIMS> {
	type Item = &'a mut T;
	type IntoIter = slice::IterMut<'a, T>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.vals.iter_mut()
	}
}

#[cfg(test)]
mod tests {
	use crate::typedefs::Vec3i32;

	#[test]
	fn iter() {
		let vec: Vec3i32 = [1, 1, 1].into();
		for e in vec {
			assert_eq!(e, 1);
		}
	}

	#[test]
	fn iter_mut() {
		let mut vec: Vec3i32 = [0, 0, 0].into();
		for e in &mut vec {
			*e = 1;
		}

		for e in vec {
			assert_eq!(e, 1);
		}
	}

	#[test]
	fn iter_ref() {
		let vec: Vec3i32 = [1, 1, 1].into();
		for e in &vec {
			assert_eq!(*e, 1);
		}
	}
}
