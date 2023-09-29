//! This module contains implementations for Quaternion<T> on Vector<T,3> mathematical operations

use std::ops::{Add, Mul, Sub};

use crate::Quaternion;
use vector::Vector;

impl<T> From<Vector<T, 4>> for Quaternion<T>
where
	T: Copy,
{
	#[inline]
	fn from(value: Vector<T, 4>) -> Self {
		value.into()
	}
}

impl<T> From<Quaternion<T>> for Vector<T, 4>
where
	T: Copy,
{
	#[inline]
	fn from(value: Quaternion<T>) -> Self {
		[value.x, value.y, value.z, value.w].into()
	}
}

impl<T> Mul<Vector<T, 3>> for Quaternion<T>
where
	T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Default + Copy,
{
	type Output = Vector<T, 3>;

	fn mul(self, rhs: Vector<T, 3>) -> Self::Output {
		let w = self.w;
		let b = Vector::from_array([self.x, self.y, self.z]);
		let b2 = b.length_squared();
		rhs * (w * w - b2) + (b * (rhs.dot(b) + (rhs.dot(b)))) + (b.cross(rhs) * (w + w))
	}
}

#[cfg(test)]
mod tests {
	use vector::Vector;

	use crate::Quaternion;

	#[test]
	fn mul_vec3_id() {
		let q_arr = [0.0, 0.0, 0.0, 1.0];
		let v_arr = [1.0, 1.0, 1.0];
		let q1 = Quaternion::from_array(q_arr);
		let v1 = Vector::from_array(v_arr);

		let r = q1 * v1;

		assert_eq!([1.0, 1.0, 1.0], r.to_array());
	}

	#[test]
	fn mul_vec3() {
		let q_arr = [1.0, 0.0, 1.0, 5.0];
		let v_arr = [2.0, 1.0, 1.0];
		let q1 = Quaternion::from_array(q_arr);
		let v1 = Vector::from_array(v_arr);

		let r = q1 * v1;

		assert_eq!([42.0, 33.0, 39.0], r.to_array());
	}
}
