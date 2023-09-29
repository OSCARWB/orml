//! This module contains implementations of mathematical operations on
//! Vectors of the same Dimension

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign};

use num_traits::identities::One;
use traits::fns::SquareRoot;

use super::Vector;

impl<T, const DIMS: usize> Vector<T, DIMS>
where
	T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
	/// The dot product between 2 vectors of the same dimension. Also know as the scalar product
	#[inline]
	pub fn dot(&self, rhs: Self) -> T {
		std::iter::zip(self.iter(), rhs.iter())
			.fold(Default::default(), |acc: T, x| acc + (*x.0 * *x.1))
	}

	/// Returns the length of the vector squared
	#[inline]
	pub fn length_squared(&self) -> T {
		self.dot(*self)
	}
}

impl<T, const DIMS: usize> Vector<T, DIMS>
where
	T: Add<Output = T> + Mul<Output = T> + Copy + Default + SquareRoot,
{
	/// Returns the length of the vector
	#[inline]
	pub fn length(&self) -> T {
		self.length_squared().sqrt()
	}
}

impl<T, const DIMS: usize> Vector<T, DIMS>
where
	T: Div<Output = T> + One,
{
	/// Returns the reciprocal of the vector. ie 1/x for each element x of the vector
	#[inline]
	pub fn reciprocal(self) -> Self {
		Self {
			vals: self.vals.map(|x| T::one() / x),
		}
	}
}

impl<T, const DIMS: usize> Vector<T, DIMS>
where
	T: Add<Output = T> + Mul<Output = T> + Copy + Default + SquareRoot + Div<Output = T> + One,
{
	/// Returns the vector normalised to length 1
	pub fn normalise(self) -> Self {
		self * (T::one() / self.length())
	}
}

impl<T> Vector<T, 3>
where
	T: Sub<Output = T> + Mul<Output = T> + Default + Copy,
{
	/// The cross product between 2 vectors of dimension 3
	#[inline]
	pub fn cross(&self, rhs: Self) -> Self {
		let a = self;
		let b = rhs;
		Self {
			vals: [
				(a[1] * b[2]) - (a[2] * b[1]),
				(a[2] * b[0]) - (a[0] * b[2]),
				(a[0] * b[1]) - (a[1] * b[0]),
			],
		}
	}
}

// Add Impl
impl<T, const DIMS: usize> Add for Vector<T, DIMS>
where
	T: Add<Output = T> + Default + Copy,
{
	type Output = Self;

	#[inline]
	fn add(self, rhs: Self) -> Self::Output {
		let mut temp = [Default::default(); DIMS];
		for (i, e) in temp.iter_mut().enumerate() {
			*e = self.vals[i] + rhs.vals[i];
		}
		Self { vals: temp }
	}
}

// AddAssign Impl
impl<T: Add<Output = T>, const DIMS: usize> AddAssign for Vector<T, DIMS>
where
	T: Default + Copy,
{
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

// AddT Impl
impl<T, const DIMS: usize> Add<T> for Vector<T, DIMS>
where
	T: Add<Output = T> + Default + Copy,
{
	type Output = Self;

	#[inline]
	fn add(self, rhs: T) -> Self::Output {
		Self {
			vals: self.vals.map(|e| e + rhs),
		}
	}
}

// AddAssignT Impl
impl<T: Add<Output = T>, const DIMS: usize> AddAssign<T> for Vector<T, DIMS>
where
	T: Default + Copy,
{
	#[inline]
	fn add_assign(&mut self, rhs: T) {
		*self = *self + rhs;
	}
}

// Sub Impl
impl<T, const DIMS: usize> Sub for Vector<T, DIMS>
where
	T: Sub<Output = T> + Default + Copy,
{
	type Output = Self;

	#[inline]
	fn sub(self, rhs: Self) -> Self::Output {
		let mut temp = [Default::default(); DIMS];
		for (i, e) in temp.iter_mut().enumerate() {
			*e = self.vals[i] - rhs.vals[i];
		}
		Self { vals: temp }
	}
}

// SubAssign Impl
impl<T, const DIMS: usize> SubAssign for Vector<T, DIMS>
where
	T: Sub<Output = T> + Default + Copy,
{
	#[inline]
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

// SubT Impl
impl<T, const DIMS: usize> Sub<T> for Vector<T, DIMS>
where
	T: Sub<Output = T> + Default + Copy,
{
	type Output = Self;

	#[inline]
	fn sub(self, rhs: T) -> Self::Output {
		Self {
			vals: self.vals.map(|e| e - rhs),
		}
	}
}

// SubAssignT Impl
impl<T, const DIMS: usize> SubAssign<T> for Vector<T, DIMS>
where
	T: Sub<Output = T> + Default + Copy,
{
	#[inline]
	fn sub_assign(&mut self, rhs: T) {
		*self = *self - rhs;
	}
}

impl<T, const DIMS: usize> Mul for Vector<T, DIMS>
where
	T: Mul<Output = T> + Default + Copy,
{
	type Output = Self;

	#[inline]
	fn mul(self, rhs: Self) -> Self::Output {
		let mut temp = [Default::default(); DIMS];
		for (i, e) in temp.iter_mut().enumerate() {
			*e = self.vals[i] * rhs.vals[i];
		}
		Self { vals: temp }
	}
}

impl<T, const DIMS: usize> MulAssign for Vector<T, DIMS>
where
	T: Mul<Output = T> + Default + Copy,
{
	#[inline]
	fn mul_assign(&mut self, rhs: Self) {
		*self = *self * rhs;
	}
}

// MulT Impl
impl<T, const DIMS: usize> Mul<T> for Vector<T, DIMS>
where
	T: Mul<Output = T> + Default + Copy,
{
	type Output = Self;

	#[inline]
	fn mul(self, rhs: T) -> Self::Output {
		Self {
			vals: self.vals.map(|e| e * rhs),
		}
	}
}

// MulAssignT Impl
impl<T, const DIMS: usize> MulAssign<T> for Vector<T, DIMS>
where
	T: Mul<Output = T> + Default + Copy,
{
	#[inline]
	fn mul_assign(&mut self, rhs: T) {
		*self = *self * rhs;
	}
}

impl<T, const DIMS: usize> Div for Vector<T, DIMS>
where
	T: Div<Output = T> + Default + Copy,
{
	type Output = Self;

	#[inline]
	fn div(self, rhs: Self) -> Self::Output {
		let mut temp = [Default::default(); DIMS];
		for (i, e) in temp.iter_mut().enumerate() {
			*e = self.vals[i] / rhs.vals[i];
		}
		Self { vals: temp }
	}
}

impl<T, const DIMS: usize> DivAssign for Vector<T, DIMS>
where
	T: Div<Output = T> + Default + Copy,
{
	#[inline]
	fn div_assign(&mut self, rhs: Self) {
		*self = *self / rhs;
	}
}

// DivT Impl
impl<T, const DIMS: usize> Div<T> for Vector<T, DIMS>
where
	T: Div<Output = T> + Default + Copy,
{
	type Output = Self;

	#[inline]
	fn div(self, rhs: T) -> Self::Output {
		Self {
			vals: self.vals.map(|e| e / rhs),
		}
	}
}

// DivAssignT Impl
impl<T, const DIMS: usize> DivAssign<T> for Vector<T, DIMS>
where
	T: Div<Output = T> + Default + Copy,
{
	#[inline]
	fn div_assign(&mut self, rhs: T) {
		*self = *self / rhs;
	}
}

// Neg Implmentation
impl<T, const DIMS: usize> Neg for Vector<T, DIMS>
where
	T: Neg<Output = T> + Default + Copy,
{
	type Output = Self;

	#[inline]
	fn neg(self) -> Self::Output {
		let mut temp = self.vals;
		for (i, e) in temp.iter_mut().enumerate() {
			*e = -self.vals[i];
		}
		Self { vals: temp }
	}
}

// Not Implmentation
impl<T, const DIMS: usize> Not for Vector<T, DIMS>
where
	T: Not<Output = T> + Default + Copy,
{
	type Output = Self;

	#[inline]
	fn not(self) -> Self::Output {
		let mut temp = self.vals;
		for (i, e) in temp.iter_mut().enumerate() {
			*e = !self.vals[i];
		}
		Self { vals: temp }
	}
}

#[cfg(test)]
mod tests {

	use crate::{
		typedefs::{Vec3Bool, Vec3i32},
		Vector,
	};

	#[test]
	fn add() {
		let vec1_1: Vec3i32 = [1, 1, 1].into();
		let vec1_2: Vec3i32 = [2, 2, 2].into();
		let expected1: Vec3i32 = [3, 3, 3].into();

		assert_eq!(expected1, vec1_1 + vec1_2);
	}

	#[test]
	fn add_assign() {
		let vec1_1: Vec3i32 = [1, 1, 1].into();
		let vec1_2: Vec3i32 = [2, 2, 2].into();
		let expected1: Vec3i32 = [3, 3, 3].into();

		let mut vec1_3 = vec1_1;
		vec1_3 += vec1_2;

		assert_eq!(expected1, vec1_3);
	}

	#[test]
	fn add_t() {
		let vec1_1: Vec3i32 = [1, 1, 1].into();
		let expected1: Vec3i32 = [3, 3, 3].into();

		assert_eq!(expected1, vec1_1 + 2);
	}

	#[test]
	fn add_assign_t() {
		let vec1_1: Vec3i32 = [1, 1, 1].into();
		let expected1: Vec3i32 = [3, 3, 3].into();

		let mut vec1_3 = vec1_1;
		vec1_3 += 2;

		assert_eq!(expected1, vec1_3);
	}

	#[test]
	fn sub() {
		let vec1_1: Vec3i32 = [3, 3, 3].into();
		let vec1_2: Vec3i32 = [2, 2, 2].into();
		let expected1: Vec3i32 = [1, 1, 1].into();

		assert_eq!(expected1, vec1_1 - vec1_2);
	}

	#[test]
	fn sub_assign() {
		let vec1_1: Vec3i32 = [3, 3, 3].into();
		let vec1_2: Vec3i32 = [2, 2, 2].into();
		let expected1: Vec3i32 = [1, 1, 1].into();

		let mut vec1_3 = vec1_1;
		vec1_3 -= vec1_2;

		assert_eq!(expected1, vec1_3);
	}

	#[test]
	fn sub_t() {
		let vec1_1: Vec3i32 = [3, 3, 3].into();
		let expected1: Vec3i32 = [1, 1, 1].into();

		assert_eq!(expected1, vec1_1 - 2);
	}

	#[test]
	fn sub_assign_t() {
		let vec1_1: Vec3i32 = [3, 3, 3].into();
		let expected1: Vec3i32 = [1, 1, 1].into();

		let mut vec1_3 = vec1_1;
		vec1_3 -= 2;

		assert_eq!(expected1, vec1_3);
	}

	#[test]
	fn mul() {
		let vec1_1: Vec3i32 = [1, 1, 1].into();
		let vec1_2: Vec3i32 = [2, 2, 2].into();
		let expected1: Vec3i32 = [2, 2, 2].into();

		assert_eq!(expected1, vec1_1 * vec1_2);
	}

	#[test]
	fn mul_assign() {
		let vec1_1: Vec3i32 = [1, 1, 1].into();
		let vec1_2: Vec3i32 = [2, 2, 2].into();
		let expected1: Vec3i32 = [2, 2, 2].into();

		let mut vec1_3 = vec1_1;
		vec1_3 *= vec1_2;

		assert_eq!(expected1, vec1_3);
	}

	#[test]
	fn mul_t() {
		let vec1_1: Vec3i32 = [1, 1, 1].into();
		let expected1: Vec3i32 = [2, 2, 2].into();

		assert_eq!(expected1, vec1_1 * 2);
	}

	#[test]
	fn mul_assign_t() {
		let vec1_1: Vec3i32 = [1, 1, 1].into();
		let expected1: Vec3i32 = [2, 2, 2].into();

		let mut vec1_3 = vec1_1;
		vec1_3 *= 2;

		assert_eq!(expected1, vec1_3);
	}

	#[test]
	fn div() {
		let vec1_1: Vec3i32 = [2, 2, 2].into();
		let vec1_2: Vec3i32 = [2, 2, 2].into();
		let expected1: Vec3i32 = [1, 1, 1].into();

		assert_eq!(expected1, vec1_1 / vec1_2);
	}

	#[test]
	fn div_assign() {
		let vec1_1: Vec3i32 = [2, 2, 2].into();
		let vec1_2: Vec3i32 = [2, 2, 2].into();
		let expected1: Vec3i32 = [1, 1, 1].into();

		let mut vec1_3 = vec1_1;
		vec1_3 /= vec1_2;

		assert_eq!(expected1, vec1_3);
	}

	#[test]
	fn div_t() {
		let vec1_1: Vec3i32 = [2, 2, 2].into();
		let expected1: Vec3i32 = [1, 1, 1].into();

		assert_eq!(expected1, vec1_1 / 2);
	}

	#[test]
	fn div_assign_t() {
		let vec1_1: Vec3i32 = [2, 2, 2].into();
		let expected1: Vec3i32 = [1, 1, 1].into();

		let mut vec1_3 = vec1_1;
		vec1_3 /= 2;

		assert_eq!(expected1, vec1_3);
	}

	#[test]
	fn neg() {
		let vec1_1: Vec3i32 = [1, 1, 1].into();
		let expected1: Vec3i32 = [-1, -1, -1].into();

		assert_eq!(expected1, -vec1_1);
	}

	#[test]
	fn not() {
		let combos: [Vec3Bool; 8] = [
			[false, false, false].into(),
			[false, false, true].into(),
			[false, true, false].into(),
			[false, true, true].into(),
			[true, false, false].into(),
			[true, false, true].into(),
			[true, true, false].into(),
			[true, true, true].into(),
		];

		for e in combos {
			let dn = !!e;
			assert_eq!(e, dn, "\n\ti: {:?} j: {:?}", e, dn);
		}
	}

	#[test]
	fn dot() {
		let vec1: Vec3i32 = [1, 3, -5].into();
		let vec2: Vec3i32 = [4, -2, -1].into();
		let expected1 = 3;
		assert_eq!(expected1, vec1.dot(vec2));

		let expected2 = 35;
		assert_eq!(expected2, vec1.dot(vec1));
	}

	#[test]
	fn cross3() {
		let vec1: Vec3i32 = [1, 0, 0].into();
		let vec2: Vec3i32 = [0, 0, 1].into();
		let expected1: Vec3i32 = [0, -1, 0].into();
		assert_eq!(expected1, vec1.cross(vec2));
	}

	#[test]
	fn length_squared() {
		let expected1: Vec3i32 = [1, 1, 1].into();
		assert_eq!(expected1.length_squared(), 3);

		let expected2: Vec3i32 = [2, 2, 2].into();
		assert_eq!(expected2.length_squared(), 12);
	}

	#[test]
	fn length() {
		let expected1: Vector<f64, 3> = [0.0, 0.0, 3.0].into();
		assert_eq!(expected1.length(), 3.0);

		let expected2: Vector<f32, 3> = [2.0, 2.0, 1.0].into();
		assert_eq!(expected2.length(), 3.0);
	}

	#[test]
	fn normalise() {
		let expected1: Vector<f64, 3> = [0.0, 0.0, 3.0].into();
		assert_eq!(expected1.normalise().length(), 1.0);

		let expected2: Vector<f32, 3> = [3.0, 4.0, 5.0].into();
		assert_eq!(expected2.normalise().length(), 1.0);
	}

	#[test]
	fn reciprocal() {
		let expected1: Vector<f64, 3> = [2.0, 2.0, 2.0].into();
		assert_eq!(expected1.reciprocal().to_array(), [0.5, 0.5, 0.5]);

		let expected2: Vector<f32, 3> = [2.0, 4.0, 8.0].into();
		assert_eq!(expected2.reciprocal().to_array(), [0.5, 0.25, 0.125]);
	}
}
