//! This module contains implementations of mathematical operations on
//! Vectors of the same Dimension

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign};

use traits::SquareRoot;

use super::Vector;

impl<T, const DIMS: usize> Vector<T, DIMS> {
	/// The dot product between 2 vectors of the same dimension
	// TODO: Need to finish this function
	pub fn dot(&self, _rhs: Self) -> Self {
		todo!()
	}
}

impl<T, const DIMS: usize> Vector<T, DIMS>
where
	T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
	/// Returns the length of the vector squared
	pub fn length_squared(&self) -> T {
		self.iter()
			.fold(Default::default(), |acc, x| (*x * *x) + acc)
	}
}

impl<T, const DIMS: usize> Vector<T, DIMS>
where
	T: Add<Output = T> + Copy + Default + Mul<Output = T> + SquareRoot<Output = T>,
{
	/// Returns the length of the vector
	pub fn length(&self) -> T {
		self.length_squared().sqrt()
	}
}

impl<T, const DIMS: usize> Add for Vector<T, DIMS>
where
	T: Add<Output = T> + Default + Copy,
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let mut temp = [Default::default(); DIMS];
		for (i, e) in temp.iter_mut().enumerate() {
			*e = self.vals[i] + rhs.vals[i];
		}
		Self { vals: temp }
	}
}

impl<T: Add<Output = T>, const DIMS: usize> AddAssign for Vector<T, DIMS>
where
	T: Default + Copy,
{
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl<T, const DIMS: usize> Add<T> for Vector<T, DIMS>
where
	T: Add<Output = T> + Default + Copy,
{
	type Output = Self;

	fn add(self, rhs: T) -> Self::Output {
		Self {
			vals: self.vals.map(|e| e + rhs),
		}
	}
}

impl<T: Add<Output = T>, const DIMS: usize> AddAssign<T> for Vector<T, DIMS>
where
	T: Default + Copy,
{
	fn add_assign(&mut self, rhs: T) {
		*self = *self + rhs;
	}
}

impl<T, const DIMS: usize> Sub for Vector<T, DIMS>
where
	T: Sub<Output = T> + Default + Copy,
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		let mut temp = [Default::default(); DIMS];
		for (i, e) in temp.iter_mut().enumerate() {
			*e = self.vals[i] - rhs.vals[i];
		}
		Self { vals: temp }
	}
}

impl<T, const DIMS: usize> SubAssign for Vector<T, DIMS>
where
	T: Sub<Output = T> + Default + Copy,
{
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs;
	}
}

impl<T, const DIMS: usize> Sub<T> for Vector<T, DIMS>
where
	T: Sub<Output = T> + Default + Copy,
{
	type Output = Self;

	fn sub(self, rhs: T) -> Self::Output {
		Self {
			vals: self.vals.map(|e| e - rhs),
		}
	}
}

impl<T, const DIMS: usize> SubAssign<T> for Vector<T, DIMS>
where
	T: Sub<Output = T> + Default + Copy,
{
	fn sub_assign(&mut self, rhs: T) {
		*self = *self - rhs;
	}
}

impl<T, const DIMS: usize> Mul<T> for Vector<T, DIMS>
where
	T: Mul<Output = T> + Default + Copy,
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self::Output {
		Self {
			vals: self.vals.map(|e| e * rhs),
		}
	}
}

impl<T, const DIMS: usize> MulAssign<T> for Vector<T, DIMS>
where
	T: Mul<Output = T> + Default + Copy,
{
	fn mul_assign(&mut self, rhs: T) {
		*self = *self * rhs;
	}
}

impl<T, const DIMS: usize> Div<T> for Vector<T, DIMS>
where
	T: Div<Output = T> + Default + Copy,
{
	type Output = Self;

	fn div(self, rhs: T) -> Self::Output {
		Self {
			vals: self.vals.map(|e| e / rhs),
		}
	}
}

impl<T, const DIMS: usize> DivAssign<T> for Vector<T, DIMS>
where
	T: Div<Output = T> + Default + Copy,
{
	fn div_assign(&mut self, rhs: T) {
		*self = *self / rhs;
	}
}

impl<T, const DIMS: usize> Neg for Vector<T, DIMS>
where
	T: Neg<Output = T> + Default + Copy,
{
	type Output = Self;

	fn neg(self) -> Self::Output {
		let mut temp = self.vals;
		for (i, e) in temp.iter_mut().enumerate() {
			*e = -self.vals[i];
		}
		Self { vals: temp }
	}
}

impl<T, const DIMS: usize> Not for Vector<T, DIMS>
where
	T: Not<Output = T> + Default + Copy,
{
	type Output = Self;

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
		typedefs::{VBool3, Vec3i32},
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
		let combos: [VBool3; 8] = [
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

		let expected2: Vector<f32, 3> = [0.0, 0.0, 3.0].into();
		assert_eq!(expected2.length(), 3.0);
	}
}

// #[test]
// fn mul() {
// 	let vec1_1: VUsizeN = [1, 1, 1].into();
// 	let vec1_2: VUsizeN = [2, 2, 2].into();
// 	let expected1: VUsizeN = [2, 2, 2].into();

// 	assert_eq!(expected1, vec1_1 * vec1_2);
// }

// #[test]
// fn mul_assign() {
// 	let vec1_1: VUsizeN = [1, 1, 1].into();
// 	let vec1_2: VUsizeN = [2, 2, 2].into();
// 	let expected1: VUsizeN = [2, 2, 2].into();

// 	let mut vec1_3 = vec1_1;
// 	vec1_3 *= vec1_2;

// 	assert_eq!(expected1, vec1_3);
// }

// #[test]
// fn div() {
// 	let vec1_1: VUsizeN = [2, 2, 2].into();
// 	let vec1_2: VUsizeN = [2, 2, 2].into();
// 	let expected1: VUsizeN = [1, 1, 1].into();

// 	assert_eq!(expected1, vec1_1 / vec1_2);
// }

// #[test]
// fn div_assign() {
// 	let vec1_1: VUsizeN = [2, 2, 2].into();
// 	let vec1_2: VUsizeN = [2, 2, 2].into();
// 	let expected1: VUsizeN = [1, 1, 1].into();

// 	let mut vec1_3 = vec1_1;
// 	vec1_3 /= vec1_2;

// 	assert_eq!(expected1, vec1_3);
// }

// impl<T, const DIMS: usize> Mul for Vector<T, DIMS>
// where
// 	T: Mul<Output = T> + Default + Copy,
// {
// 	type Output = Self;

// 	fn mul(self, rhs: Self) -> Self::Output {
// 		let mut temp = [Default::default(); DIMS];
// 		for (i, e) in temp.iter_mut().enumerate() {
// 			*e = self.vals[i] * rhs.vals[i];
// 		}
// 		Self { vals: temp }
// 	}
// }

// impl<T, const DIMS: usize> MulAssign for Vector<T, DIMS>
// where
// 	T: Mul<Output = T> + Default + Copy,
// {
// 	fn mul_assign(&mut self, rhs: Self) {
// 		*self = *self * rhs;
// 	}
// }

// impl<T, const DIMS: usize> Div for Vector<T, DIMS>
// where
// 	T: Div<Output = T> + Default + Copy,
// {
// 	type Output = Self;

// 	fn div(self, rhs: Self) -> Self::Output {
// 		let mut temp = [Default::default(); DIMS];
// 		for (i, e) in temp.iter_mut().enumerate() {
// 			*e = self.vals[i] / rhs.vals[i];
// 		}
// 		Self { vals: temp }
// 	}
// }

// impl<T, const DIMS: usize> DivAssign for Vector<T, DIMS>
// where
// 	T: Div<Output = T> + Default + Copy,
// {
// 	fn div_assign(&mut self, rhs: Self) {
// 		*self = *self / rhs;
// 	}
// }
