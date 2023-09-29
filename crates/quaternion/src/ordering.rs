//! This module containts implementations for ordering Quaternions

use super::Quaternion;

impl<T> PartialEq for Quaternion<T>
where
	T: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
	}
}

impl<T> Eq for Quaternion<T> where T: Eq {}

#[cfg(test)]
mod tests {
	use crate::Quaternion;

	#[test]
	fn partial_eq() {
		let combos: [Quaternion<bool>; 16] = [
			[false, false, false, false].into(),
			[false, false, false, true].into(),
			[false, false, true, false].into(),
			[false, false, true, true].into(),
			[false, true, false, false].into(),
			[false, true, false, true].into(),
			[false, true, true, false].into(),
			[false, true, true, true].into(),
			[true, false, false, false].into(),
			[true, false, false, true].into(),
			[true, false, true, false].into(),
			[true, false, true, true].into(),
			[true, true, false, false].into(),
			[true, true, false, true].into(),
			[true, true, true, false].into(),
			[true, true, true, true].into(),
		];

		for i in 0..combos.len() {
			for j in 0..combos.len() {
				assert_eq!(
					combos[i] == combos[j],
					i == j,
					"\n\ti: {:?} j: {:?}",
					combos[i],
					combos[j]
				);
			}
		}
	}
}
