use std::ops::{
	Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use fns::{
	abs::AbsFns, exp::ExpFns, float::FloatFns, log::LogFns, pow::Pow, trig::TrigFns, Ceil, Clamp,
	CubeRoot, Degrees, DivEuclid, Floor, Fract, Hypot, Max, Min, MulAdd, Recip, RemEuclid, Round,
	Signed, SquareRoot, Trunc,
};

pub trait Ops:
	Add
	+ AddAssign
	+ Div
	+ DivAssign
	+ Mul
	+ MulAssign
	+ Neg
	+ Rem
	+ RemAssign
	+ Sub
	+ SubAssign
	+ std::marker::Sized
{
}

pub trait Float:
	Ops
	+ TrigFns
	+ LogFns
	+ FloatFns
	+ ExpFns
	+ AbsFns
	+ CubeRoot
	+ Ceil
	+ Clamp
	+ DivEuclid
	+ Floor
	+ Fract
	+ Hypot
	+ Signed
	+ Max
	+ Min
	+ MulAdd
	+ Pow
	+ Recip
	+ RemEuclid
	+ Round
	+ SquareRoot
	+ Degrees
	+ Trunc
{
}

pub mod fns;

#[cfg(feature = "impls")]
pub mod impls {
	#[cfg(feature = "impl-f64")]
	pub mod f64 {
		use crate::fns::{trig::*, SquareRoot};

		use crate::impl_trivial_trig;
		use crate::impl_trivial_trig_all;

		impl_trivial_trig_all!(f64);

		impl SquareRoot for f64 {
			fn sqrt(&self) -> Self {
				f64::sqrt(*self)
			}
		}
	}

	#[cfg(feature = "impl-f32")]
	pub mod f32 {
		use crate::fns::{trig::*, SquareRoot};

		use crate::impl_trivial_trig;
		use crate::impl_trivial_trig_all;

		impl_trivial_trig_all!(f32);

		impl SquareRoot for f32 {
			fn sqrt(&self) -> Self {
				f32::sqrt(*self)
			}
		}
	}
}
