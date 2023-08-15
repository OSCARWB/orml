use std::ops::{Add, Div, Mul, MulAssign, Neg, Sub};

use crate::Quaternion;

impl<T> Add<Quaternion<T>> for Quaternion<T>
where
	T: Default + Copy + Add<Output = T>,
{
	type Output = Self;

	fn add(self, rhs: Quaternion<T>) -> Self::Output {
		self.zip(rhs)
			.enumerate()
			.fold([Default::default(); 4], |mut acc: [T; 4], x| {
				acc[x.0] = x.1 .0 + x.1 .1;
				acc
			})
			.into()
	}
}

impl<T> Sub<Quaternion<T>> for Quaternion<T>
where
	T: Default + Copy + Sub<Output = T>,
{
	type Output = Self;

	fn sub(self, rhs: Quaternion<T>) -> Self::Output {
		self.zip(rhs)
			.enumerate()
			.fold([Default::default(); 4], |mut acc: [T; 4], x| {
				acc[x.0] = x.1 .0 - x.1 .1;
				acc
			})
			.into()
	}
}

impl<T> Mul<Quaternion<T>> for Quaternion<T>
where
	T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
	type Output = Self;

	fn mul(self, rhs: Quaternion<T>) -> Self::Output {
		let l = self;
		let r = rhs;
		[
			(l.w * r.x) + (l.x * r.w) + (l.y * r.z) - (l.z * r.y),
			(l.w * r.y) - (l.x * r.z) + (l.y * r.w) + (l.z * r.x),
			(l.w * r.z) + (l.x * r.y) - (l.y * r.x) + (l.z * r.w),
			(l.w * r.w) - (l.x * r.x) - (l.y * r.y) - (l.z * r.z),
		]
		.into()
	}
}

impl<T> MulAssign<Quaternion<T>> for Quaternion<T>
where
	T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
	fn mul_assign(&mut self, rhs: Quaternion<T>) {
		*self = *self * rhs;
	}
}

impl<T> Mul<T> for Quaternion<T>
where
	T: Mul<Output = T> + Copy,
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self::Output {
		self.to_array().map(|x| x * rhs).into()
	}
}

impl<T> Div<T> for Quaternion<T>
where
	T: Div<Output = T> + Copy,
{
	type Output = Self;

	fn div(self, rhs: T) -> Self::Output {
		self.to_array().map(|x| x / rhs).into()
	}
}

impl<T> Neg for Quaternion<T>
where
	T: Neg<Output = T> + Copy,
{
	type Output = Self;

	fn neg(self) -> Self::Output {
		self.to_array().map(|x| -x).into()
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
}
