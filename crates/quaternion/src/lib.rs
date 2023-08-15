// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
#![allow(dead_code)]

pub mod arithmetic;
#[cfg(feature = "vector")]
pub mod vector;

pub mod ordering;

#[derive(Debug)]
pub struct Quaternion<T> {
	pub x: T,
	pub y: T,
	pub z: T,
	pub w: T,
}

impl<T> Quaternion<T> {
	pub fn zip(
		self,
		rhs: Self,
	) -> std::iter::Zip<std::array::IntoIter<T, 4>, std::array::IntoIter<T, 4>> {
		std::iter::zip::<[T; 4], [T; 4]>(self.into(), rhs.into())
	}

	pub fn to_array(self) -> [T; 4] {
		self.into()
	}
}

impl<T> Quaternion<T>
where
	T: Copy,
{
	pub fn from_array(arr: [T; 4]) -> Quaternion<T> {
		arr.into()
	}
}

impl<T> From<[T; 4]> for Quaternion<T>
where
	T: Copy,
{
	fn from(value: [T; 4]) -> Self {
		Self {
			x: value[0],
			y: value[1],
			z: value[2],
			w: value[3],
		}
	}
}

impl<T> From<Quaternion<T>> for [T; 4] {
	fn from(value: Quaternion<T>) -> Self {
		[value.x, value.y, value.z, value.w]
	}
}

impl<T> Clone for Quaternion<T>
where
	T: Clone,
{
	fn clone(&self) -> Self {
		Self {
			x: self.x.clone(),
			y: self.y.clone(),
			z: self.z.clone(),
			w: self.w.clone(),
		}
	}
}

impl<T> Copy for Quaternion<T> where T: Copy {}
