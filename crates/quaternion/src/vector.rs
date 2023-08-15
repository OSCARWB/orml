use std::ops::{Add, Mul, Sub};

use crate::Quaternion;
use vector::Vector;

impl<T> From<Vector<T, 4>> for Quaternion<T>
where
	T: Copy,
{
	fn from(value: Vector<T, 4>) -> Self {
		value.into()
	}
}

impl<T> From<Quaternion<T>> for Vector<T, 4>
where
	T: Copy,
{
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
	use crate::Quaternion;

	#[test]
	fn mul_vec3() {
		let q1 = Quaternion::from_array([0.0, 0.0, 0.0, 1.0]);
	}
}
