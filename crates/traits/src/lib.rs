pub trait SquareRoot {
	type Output;
	fn sqrt(&self) -> Self::Output;
}

pub mod impls {
	pub mod f64 {
		use crate::SquareRoot;

		impl SquareRoot for f64 {
			type Output = f64;

			fn sqrt(&self) -> Self::Output {
				f64::sqrt(*self)
			}
		}
	}

	pub mod f32 {
		use crate::SquareRoot;

		impl SquareRoot for f32 {
			type Output = f32;

			fn sqrt(&self) -> Self::Output {
				f32::sqrt(*self)
			}
		}
	}
}
