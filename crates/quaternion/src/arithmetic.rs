//! This module contains implementations of mathematical operations on Quaternions

use std::ops::{Add, Div, Mul, MulAssign, Neg, Sub};

use crate::Quaternion;
use num_traits::One;
use traits::fns::SquareRoot;

impl<T> Quaternion<T>
where
	T:  Add<Output = T>,
	for<'a> &'a T: Mul<&'a T, Output = T>,
{
	/// The dot product between 2 Quaternion of the same dimension. Also know as the scalar product
	#[inline]
	pub fn dot(&self, rhs: &Self) -> T {
		&self.x * &rhs.x + &self.y * &rhs.y + &self.z * &rhs.z + &self.w * &rhs.w
	}

	/// Returns the length of the Quaternion squared
	#[inline]
	pub fn length_squared(&self) -> T {
		self.dot(self)
	}
}

impl<T> Quaternion<T>
where
	T: Add<Output = T> + SquareRoot,
	for<'a> &'a T: Mul<&'a T, Output = T>,
{
	/// Returns the length of the Quaternion
	#[inline]
	pub fn length(&self) -> T {
		self.length_squared().sqrt()
	}
}

impl<T> Quaternion<T>
where
	T: Div<Output = T> + One,
{
	/// Returns the reciprocal of the vector. ie 1/x for each element x of the vector
	#[inline]
	pub fn reciprocal(self) -> Self {
		Self {
			x: T::one() / self.x,
			y: T::one() / self.y,
			z: T::one() / self.z,
			w: T::one() / self.w,
		}
	}
}

impl<T> Quaternion<T>
where
T: Add<Output = T> + SquareRoot + One + Div<Output = T>,
for<'a> &'a T: Mul<&'a T, Output = T>,
{
	/// Returns the Quaternion normalised to length 1
	pub fn normalise(self) -> Self {
		let len = self.length();
		self * (T::one() / len)
	}
}

impl<T> Add<Quaternion<T>> for Quaternion<T>
where
	T: Add<Output = T>,
{
	type Output = Self;

	fn add(self, rhs: Quaternion<T>) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
			w: self.w + rhs.w,
		}
	}
}

impl<T> Sub<Quaternion<T>> for Quaternion<T>
where
	T: Sub<Output = T>,
{
	type Output = Self;

	fn sub(self, rhs: Quaternion<T>) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
			w: self.w - rhs.w,
		}
	}
}

impl<T> Mul<Quaternion<T>> for Quaternion<T>
where
	T: Add<Output = T> + Sub<Output = T>,
	for<'a> &'a T: Mul<&'a T, Output = T>,
{
	type Output = Self;

	fn mul(self, rhs: Quaternion<T>) -> Self::Output {
		let l = self;
		let r = rhs;
		Self {
			x: (&l.w * &r.x) + (&l.x * &r.w) + (&l.y * &r.z) - (&l.z * &r.y),
			y: (&l.w * &r.y) - (&l.x * &r.z) + (&l.y * &r.w) + (&l.z * &r.x),
			z: (&l.w * &r.z) + (&l.x * &r.y) - (&l.y * &r.x) + (&l.z * &r.w),
			w: (&l.w * &r.w) - (&l.x * &r.x) - (&l.y * &r.y) - (&l.z * &r.z),
		}
	}
}

impl<T> Mul<&Quaternion<T>> for Quaternion<T>
where
	T: Add<Output = T> + Sub<Output = T>,
	for<'a> &'a T: Mul<&'a T, Output = T>,
{
	type Output = Quaternion<T>;

	fn mul(self, rhs: &Quaternion<T>) -> Self::Output {
		let l = self;
		let r = rhs;
		Self::Output {
			x: (&l.w * &r.x) + (&l.x * &r.w) + (&l.y * &r.z) - (&l.z * &r.y),
			y: (&l.w * &r.y) - (&l.x * &r.z) + (&l.y * &r.w) + (&l.z * &r.x),
			z: (&l.w * &r.z) + (&l.x * &r.y) - (&l.y * &r.x) + (&l.z * &r.w),
			w: (&l.w * &r.w) - (&l.x * &r.x) - (&l.y * &r.y) - (&l.z * &r.z),
		}
	}
}

impl<T> Mul<&Quaternion<T>> for &Quaternion<T>
where
	T: Add<Output = T> + Sub<Output = T>,
	for<'a> &'a T: Mul<&'a T, Output = T>,
{
	type Output = Quaternion<T>;

	fn mul(self, rhs: &Quaternion<T>) -> Self::Output {
		let l = self;
		let r = rhs;
		Self::Output {
			x: (&l.w * &r.x) + (&l.x * &r.w) + (&l.y * &r.z) - (&l.z * &r.y),
			y: (&l.w * &r.y) - (&l.x * &r.z) + (&l.y * &r.w) + (&l.z * &r.x),
			z: (&l.w * &r.z) + (&l.x * &r.y) - (&l.y * &r.x) + (&l.z * &r.w),
			w: (&l.w * &r.w) - (&l.x * &r.x) - (&l.y * &r.y) - (&l.z * &r.z),
		}
	}
}

impl<'b,T> Mul<Quaternion<T>> for &'b Quaternion<T>
where
	T: Add<Output = T> + Sub<Output = T>,
	for<'a> &'a T: Mul<&'a T, Output = T>,
{
	type Output = Quaternion<T>;

	fn mul(self, rhs: Quaternion<T>) -> Self::Output {
		let l = self;
		let r = rhs;
		Self::Output {
			x: (&l.w * &r.x) + (&l.x * &r.w) + (&l.y * &r.z) - (&l.z * &r.y),
			y: (&l.w * &r.y) - (&l.x * &r.z) + (&l.y * &r.w) + (&l.z * &r.x),
			z: (&l.w * &r.z) + (&l.x * &r.y) - (&l.y * &r.x) + (&l.z * &r.w),
			w: (&l.w * &r.w) - (&l.x * &r.x) - (&l.y * &r.y) - (&l.z * &r.z),
		}
	}
}

impl<T> Mul<Quaternion<T>> for & mut Quaternion<T>
where
	T: Add<Output = T> + Sub<Output = T>,
	for<'a> &'a T: Mul<&'a T, Output = T>,
{
	type Output = Quaternion<T>;

	fn mul(self, rhs: Quaternion<T>) -> Self::Output {
		let l = self;
		let r = rhs;
		Self::Output {
			x: (&l.w * &r.x) + (&l.x * &r.w) + (&l.y * &r.z) - (&l.z * &r.y),
			y: (&l.w * &r.y) - (&l.x * &r.z) + (&l.y * &r.w) + (&l.z * &r.x),
			z: (&l.w * &r.z) + (&l.x * &r.y) - (&l.y * &r.x) + (&l.z * &r.w),
			w: (&l.w * &r.w) - (&l.x * &r.x) - (&l.y * &r.y) - (&l.z * &r.z),
		}
	}
}

impl<T> MulAssign<Quaternion<T>> for Quaternion<T>
where
	T: Add<Output = T> + Sub<Output = T> + Clone,
	for<'a> &'a T: Mul<&'a T, Output = T>,
{
	fn mul_assign(&mut self, rhs: Quaternion<T>) {
		*self = self.clone() * rhs;
	}
}

impl<T> Mul<T> for Quaternion<T>
where
	for<'a> &'a T: Mul<&'a T, Output = T>
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self::Output {
		Self {
			x: &self.x * &rhs,
			y: &self.y * &rhs,
			z: &self.z * &rhs,
			w: &self.w * &rhs,
		}
	}
}

impl<T> Div<T> for Quaternion<T>
where
	for<'a> &'a T: Div<&'a T, Output = T>
{
	type Output = Self;

	fn div(self, rhs: T) -> Self::Output {
		Self {
			x: &self.x / &rhs,
			y: &self.y / &rhs,
			z: &self.z / &rhs,
			w: &self.w / &rhs,
		}
	}
}

impl<T> Neg for Quaternion<T>
where
	T: Neg<Output = T>,
{
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
			w: -self.w,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::Quaternion;

	#[test]
	fn add() {
		let q1 = Quaternion::from_array([1.0, 1.0, 1.0, 1.0]);
		let q2 = Quaternion::from_array([2.0, 2.0, 2.0, 2.0]);
		let e1 = Quaternion::from_array([3.0, 3.0, 3.0, 3.0]);

		assert_eq!(e1, q1 + q2);
	}

	#[test]
	fn sub() {
		let q1 = Quaternion::from_array([3.0, 3.0, 3.0, 3.0]);
		let q2 = Quaternion::from_array([1.0, 1.0, 1.0, 1.0]);
		let e1 = Quaternion::from_array([2.0, 2.0, 2.0, 2.0]);

		assert_eq!(e1, q1 - q2);
	}

	#[test]
	fn mul() {
		let q1 = Quaternion::from_array([1.0, 1.0, 1.0, 1.0]);
		let q2 = Quaternion::from_array([2.0, 2.0, 2.0, 2.0]);
		let e1 = Quaternion::from_array([4.0, 4.0, 4.0, -4.0]);

		assert_eq!(e1, q1 * q2);
	}

	#[test]
	fn mul_assign() {
		let mut q1 = Quaternion::from_array([1.0, 1.0, 1.0, 1.0]);
		let q2 = Quaternion::from_array([2.0, 2.0, 2.0, 2.0]);
		let e1 = Quaternion::from_array([4.0, 4.0, 4.0, -4.0]);

		q1 *= q2;

		assert_eq!(e1, q1);
	}

	#[test]
	fn mul_t() {
		let q1 = Quaternion::from_array([1.0, 1.0, 1.0, 1.0]);
		let e1 = Quaternion::from_array([4.0, 4.0, 4.0, 4.0]);

		assert_eq!(e1, q1 * 4.0);
	}

	#[test]
	fn div_t() {
		let q1 = Quaternion::from_array([4.0, 4.0, 4.0, 4.0]);
		let e1 = Quaternion::from_array([1.0, 1.0, 1.0, 1.0]);

		assert_eq!(e1, q1 / 4.0);
	}

	#[test]
	fn neg() {
		let q1 = Quaternion::from_array([1.0, 1.0, 1.0, 1.0]);
		let e1 = Quaternion::from_array([-1.0, -1.0, -1.0, -1.0]);

		assert_eq!(e1, -q1);
	}

	#[test]
	fn dot() {
		let vec1: Quaternion<f32> = [1.0, 3.0, -5.0, 0.0].into();
		let vec2: Quaternion<f32> = [4.0, -2.0, -1.0, 0.0].into();
		let expected1 = 3.0;
		assert_eq!(expected1, vec1.dot(&vec2));

		let expected2 = 35.0;
		assert_eq!(expected2, vec1.dot(&vec1));
	}

	#[test]
	fn length_squared() {
		let expected1: Quaternion<f32> = [1.0, 1.0, 1.0, 0.0].into();
		assert_eq!(expected1.length_squared(), 3.0);

		let expected2: Quaternion<f32> = [2.0, 2.0, 2.0, 0.0].into();
		assert_eq!(expected2.length_squared(), 12.0);
	}

	#[test]
	fn length() {
		let expected1: Quaternion<f32> = [0.0, 0.0, 3.0, 0.0].into();
		assert_eq!(expected1.length(), 3.0);

		let expected2: Quaternion<f32> = [2.0, 2.0, 1.0, 0.0].into();
		assert_eq!(expected2.length(), 3.0);
	}

	#[test]
	fn normalise() {
		let expected1: Quaternion<f32> = [0.0, 0.0, 3.0, 0.0].into();
		assert_eq!(expected1.normalise().length(), 1.0);

		let expected2: Quaternion<f32> = [3.0, 4.0, 5.0, 0.0].into();
		assert_eq!(expected2.normalise().length(), 1.0);
	}

	#[test]
	fn reciprocal() {
		let expected1: Quaternion<f32> = [2.0, 2.0, 2.0, 1.0].into();
		assert_eq!(expected1.reciprocal().to_array(), [0.5, 0.5, 0.5, 1.0]);

		let expected2: Quaternion<f32> = [2.0, 4.0, 8.0, 1.0].into();
		assert_eq!(expected2.reciprocal().to_array(), [0.5, 0.25, 0.125, 1.0]);
	}
}
