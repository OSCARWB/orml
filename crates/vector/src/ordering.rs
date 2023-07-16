use std::ops::{Add, Mul};

use super::Vector;

impl<T, const DIMS: usize> PartialEq for Vector<T, DIMS>
where
	T: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.vals == other.vals
	}
}

impl<T, const DIMS: usize> Eq for Vector<T, DIMS> where T: Eq {}

impl<T, const DIMS: usize> PartialOrd for Vector<T, DIMS>
where
	T: Add<Output = T> + Mul<Output = T> + Default + PartialOrd + Copy,
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.length_squared().partial_cmp(&other.length_squared())
	}
}

impl<T, const DIMS: usize> Ord for Vector<T, DIMS>
where
	T: Add<Output = T> + Mul<Output = T> + Default + Ord + Copy,
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.length_squared().cmp(&other.length_squared())
	}
}

#[cfg(test)]
mod tests {
	use crate::{tests::{VUsizeN, SIZE}, typedefs::VBool3};

	#[test]
	fn partial_eq() {
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

	#[test]
	fn partial_ord() {
		let mut count = 0;
		let combos: Vec<VUsizeN> = Vec::with_capacity(SIZE)
			.into_iter()
			.map(|_: VUsizeN| {
				let temp = [count, count, count].into();
				count += 1;
				temp
			})
			.collect();

		for i in 0..combos.len() {
			for j in 0..combos.len() {
				assert_eq!(
					combos[i] < combos[j],
					i < j,
					"\n\ti: {:?} < j: {:?}",
					combos[i],
					combos[j]
				);
				assert_eq!(
					combos[i] <= combos[j],
					i <= j,
					"\n\ti: {:?} <= j: {:?}",
					combos[i],
					combos[j]
				);
				assert_eq!(
					combos[i] > combos[j],
					i > j,
					"\n\ti: {:?} < j: {:?}",
					combos[i],
					combos[j]
				);
				assert_eq!(
					combos[i] >= combos[j],
					i >= j,
					"\n\ti: {:?} >= j: {:?}",
					combos[i],
					combos[j]
				);
			}
		}
	}

	#[test]
	fn ord() {
		let mut count = 0;
		let combos: Vec<VUsizeN> = Vec::with_capacity(SIZE)
			.into_iter()
			.map(|_: VUsizeN| {
				let temp = [count, count, count].into();
				count += 1;
				temp
			})
			.collect();

		for i in 0..combos.len() {
			for j in 0..combos.len() {
				assert_eq!(
					combos[i].max(combos[j]),
					combos[i.max(j)],
					"\n\ti: {:?} max() j: {:?}",
					combos[i],
					combos[j]
				);
				assert_eq!(
					combos[i].min(combos[j]),
					combos[i.min(j)],
					"\n\ti: {:?} min() j: {:?}",
					combos[i],
					combos[j]
				);
			}
		}
	}
}
