use std::{
	fmt::{Binary, Display, Octal, UpperHex},
	ops::{
		Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
		SubAssign,
	},
	str::FromStr,
	sync::Mutex,
};

use astro_float::{BigFloat, Consts, RoundingMode};

use lazy_static::lazy_static;
use num_traits::{Num, One, Zero};
pub use orml_traits::{
	fns::{pow::Pow, trig::Acos, SquareRoot},
	impl_all_trig,
};

const P: usize = 1024;
const RM: RoundingMode = RoundingMode::None;

lazy_static! {
	static ref CC: Mutex<Consts> = Mutex::new(Consts::new().expect("Couldn't create consts cache"));
}

#[derive(Debug)]
pub struct EasyBigFloat {
	val: BigFloat,
}

impl EasyBigFloat {
	pub fn new() -> Self {
		Self {
			val: BigFloat::new(P),
		}
	}

	pub fn from_f64(f: f64) -> Self {
		Self {
			val: BigFloat::from_f64(f, P),
		}
	}
}

impl Default for EasyBigFloat {
	fn default() -> Self {
		Self::new()
	}
}

// DEREF
//
// impl Deref for EasyBigFloat {
// 	type Target = BigFloat;

// 	fn deref(&self) -> &Self::Target {
// 		&self.val
// 	}
// }

// impl DerefMut for EasyBigFloat {
// 	fn deref_mut(&mut self) -> &mut Self::Target {
// 		&mut self.val
// 	}
// }

macro_rules! impl_arith {
	($bound:ident,$fn:ident,$fn2:ident,$lhs:ty,$p:ident,$rm:ident) => {
		impl_arith!($bound, $fn, $fn2, $lhs, $lhs, $p, $rm);
	};
	($bound:ident,$fn:ident,$fn2:ident,$lhs:ty,$rhs:ty) => {
		impl $bound<$rhs> for $lhs {
			type Output = EasyBigFloat;

			fn $fn(self, rhs: $rhs) -> Self::Output {
				Self::Output {
					val: self.val.$fn2(&rhs.val),
				}
			}
		}
	};
	($bound:ident,$fn:ident,$fn2:ident,$lhs:ty) => {
		impl_arith!($bound, $fn, $fn2, $lhs, $lhs);
	};
	($bound:ident,$fn:ident,$fn2:ident,$lhs:ty,$rhs:ty,$p:ident,$rm:ident) => {
		impl $bound<$rhs> for $lhs {
			type Output = EasyBigFloat;

			fn $fn(self, rhs: $rhs) -> Self::Output {
				Self::Output {
					val: self.val.$fn2(&rhs.val, $p, $rm),
				}
			}
		}
	};
}

macro_rules! impl_arith_assign {
	($bound:ident,$fn:ident,$fn2:ident,$lhs:ty,$rhs:ty) => {
		impl $bound<$rhs> for $lhs {
			fn $fn(&mut self, rhs: $rhs) {
				self.val = self.val.$fn2(&rhs.val);
			}
		}
	};
	($bound:ident,$fn:ident,$fn2:ident,$lhs:ty,$rhs:ty,$p:ident,$rm:ident) => {
		impl $bound<$rhs> for $lhs {
			fn $fn(&mut self, rhs: $rhs) {
				self.val = self.val.$fn2(&rhs.val, $p, $rm);
			}
		}
	};
}

macro_rules! impl_arith_4 {
	($bound:ident,$fn:ident,$fn2:ident,$tt:ty) => {
		impl_arith!($bound, $fn, $fn2, $tt);
		impl_arith!($bound, $fn, $fn2, &$tt);
		impl_arith!($bound, $fn, $fn2, $tt, &$tt);
		impl_arith!($bound, $fn, $fn2, &$tt, $tt);
	};
	($bound:ident,$fn:ident,$fn2:ident,$tt:ty,$p:ident,$rm:ident) => {
		impl_arith!($bound, $fn, $fn2, $tt, $p, $rm);
		impl_arith!($bound, $fn, $fn2, &$tt, $p, $rm);
		impl_arith!($bound, $fn, $fn2, $tt, &$tt, $p, $rm);
		impl_arith!($bound, $fn, $fn2, &$tt, $tt, $p, $rm);
	};
}

// ADD
//
impl_arith_4!(Add, add, add_full_prec, EasyBigFloat);

// ADD ASSIGN
//
impl_arith_assign!(
	AddAssign,
	add_assign,
	add_full_prec,
	EasyBigFloat,
	EasyBigFloat
);
impl_arith_assign!(
	AddAssign,
	add_assign,
	add_full_prec,
	EasyBigFloat,
	&EasyBigFloat
);

// CLONE
//
impl Clone for EasyBigFloat {
	fn clone(&self) -> Self {
		Self {
			val: self.val.clone(),
		}
	}
}

// DISPLAY
macro_rules! impl_display {
	($bound:ident) => {
		impl $bound for EasyBigFloat {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				$bound::fmt(&self.val, f)
			}
		}
	};
}

impl_display!(Binary);
impl_display!(Octal);
impl_display!(UpperHex);
impl_display!(Display);

// DIV
//
impl_arith_4!(Div, div, div, EasyBigFloat, P, RM);

// DIV ASSIGN
//
impl_arith_assign!(
	DivAssign,
	div_assign,
	div,
	EasyBigFloat,
	EasyBigFloat,
	P,
	RM
);
impl_arith_assign!(
	DivAssign,
	div_assign,
	div,
	EasyBigFloat,
	&EasyBigFloat,
	P,
	RM
);

// MUL
//
impl_arith_4!(Mul, mul, mul_full_prec, EasyBigFloat);

// MUL ASSIGN
//
impl_arith_assign!(
	MulAssign,
	mul_assign,
	mul_full_prec,
	EasyBigFloat,
	EasyBigFloat
);
impl_arith_assign!(
	MulAssign,
	mul_assign,
	mul_full_prec,
	EasyBigFloat,
	&EasyBigFloat
);

// NEG
//
impl Neg for &EasyBigFloat {
	type Output = EasyBigFloat;

	fn neg(self) -> Self::Output {
		Self::Output {
			val: self.val.clone().neg(),
		}
	}
}

impl Neg for EasyBigFloat {
	type Output = EasyBigFloat;

	fn neg(self) -> Self::Output {
		Self::Output {
			val: self.val.neg(),
		}
	}
}

// PARTIAL_EQ
//
impl PartialEq for EasyBigFloat {
	fn eq(&self, other: &Self) -> bool {
		self.val == other.val
	}
}

impl Eq for EasyBigFloat {}

// PARTIAL_ORD
//
impl PartialOrd for EasyBigFloat {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.val.partial_cmp(&other.val)
	}
}

// REM
//
impl_arith_4!(Rem, rem, rem, EasyBigFloat);

// REM ASSIGN
//
impl_arith_assign!(RemAssign, rem_assign, rem, EasyBigFloat, EasyBigFloat);
impl_arith_assign!(RemAssign, rem_assign, rem, EasyBigFloat, &EasyBigFloat);

// SUB
//
impl_arith_4!(Sub, sub, sub_full_prec, EasyBigFloat);

// SUB ASSIGN
//
impl_arith_assign!(
	SubAssign,
	sub_assign,
	sub_full_prec,
	EasyBigFloat,
	EasyBigFloat
);
impl_arith_assign!(
	SubAssign,
	sub_assign,
	sub_full_prec,
	EasyBigFloat,
	&EasyBigFloat
);

impl Zero for EasyBigFloat {
	fn zero() -> Self {
		Self {
			val: BigFloat::from_i8(0, P),
		}
	}

	fn is_zero(&self) -> bool {
		*self == EasyBigFloat::zero()
	}
}

impl One for EasyBigFloat {
	fn one() -> Self {
		Self {
			val: BigFloat::from_i8(1, P),
		}
	}
}

impl Num for EasyBigFloat {
	type FromStrRadixErr = ();

	fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
		if radix != 10 {
			Ok(Self {
				val: BigFloat::from_str(str).unwrap(),
			})
		} else {
			Err(())
		}
	}
}

use orml_traits::fns::trig::*;

macro_rules! CC {
	() => {
		CC.lock().unwrap()
	};
}

fn atan2(y: &EasyBigFloat, x: &EasyBigFloat) -> EasyBigFloat {
	let zero = EasyBigFloat::zero();
	let pi = EasyBigFloat {
		val: CC!().pi(P, RM),
	};
	match (zero.val.cmp(&y.val), zero.val.cmp(&x.val)) {
		(Some(cy), Some(cx)) => {
			if (cy, cx) == (0, 0) {
				zero.clone()
			} else {
				let atanyx = y.div(x).atan();
				if cx >= 0 {
					atanyx
				} else if cy >= 0 {
					atanyx.add(pi)
				} else if cy < 0 {
					atanyx.sub(pi)
				} else {
					zero.clone()
				}
			}
		}
		(_, _) => EasyBigFloat {
			val: astro_float::NAN,
		},
	}
}

macro_rules! impl_fn {
	($fn:ident) => {
		#[inline]
		pub fn $fn(&self) -> Self {
			Self {
				val: BigFloat::$fn(&self.val, P, RM),
			}
		}
	};
	($fn:ident,$n:ty) => {
		#[inline]
		pub fn $fn(&self, n: $n) -> Self {
			Self {
				val: BigFloat::$fn(&self.val, n, P, RM),
			}
		}
	};
}

macro_rules! impl_cc_fn {
	($fn:ident) => {
		#[inline]
		pub fn $fn(&self) -> Self {
			Self {
				val: BigFloat::$fn(&self.val, P, RM, &mut CC!()),
			}
		}
	};
	($fn:ident,$n:ty) => {
		#[inline]
		pub fn $fn(&self, n: $n) -> Self {
			Self {
				val: BigFloat::$fn(&self.val, &n.val, P, RM, &mut CC!()),
			}
		}
	};
}

impl EasyBigFloat {
	impl_cc_fn!(pow, &Self);

	// Trig
	impl_cc_fn!(acos);
	impl_cc_fn!(acosh);
	impl_cc_fn!(asin);
	impl_cc_fn!(asinh);
	impl_cc_fn!(atan);
	impl_cc_fn!(atanh);
	impl_cc_fn!(cos);
	impl_cc_fn!(cosh);
	impl_cc_fn!(sin);
	impl_cc_fn!(sinh);
	impl_cc_fn!(tan);
	impl_cc_fn!(tanh);
	#[inline]
	pub fn atan2(&self, other: &Self) -> Self {
		atan2(other, self)
	}

	impl_fn!(powi, usize);
	impl_fn!(sqrt);
}

macro_rules! impl_trig {
	($bound:ident,$fn:ident) => {
		impl $bound for EasyBigFloat {
			#[inline]
			fn $fn(&self) -> Self {
				Self {
					val: BigFloat::$fn(&self.val, P, RM, &mut CC!()),
				}
			}
		}
	};
}

impl_all_trig!(impl_trig);

impl Atan2 for EasyBigFloat {
	#[inline]
	fn atan2(&self, other: &Self) -> Self {
		EasyBigFloat::atan2(other, self)
	}
}

impl Pow for EasyBigFloat {
	#[inline]
	fn pow(&self, n: &Self) -> Self {
		EasyBigFloat::pow(self, n)
	}

	#[inline]
	fn powi(&self, n: usize) -> Self {
		EasyBigFloat::powi(self, n)
	}
}

impl SquareRoot for EasyBigFloat {
	#[inline]
	fn sqrt(&self) -> Self {
		EasyBigFloat::sqrt(self)
	}
}

impl From<f64> for EasyBigFloat {
	fn from(value: f64) -> Self {
		Self {
			val: BigFloat::from(value),
		}
	}
}

#[cfg(test)]
mod tests {
	use orml_vector::Vector;

	use super::*;

	#[test]
	fn test() {
		let a = EasyBigFloat::from_f64(10.0);
		let b = EasyBigFloat::from_f64(1.0);
		let c = EasyBigFloat::from_f64(11.0);

		assert_eq!(c, a + b);
	}

	#[test]
	fn test_vec() {
		let a = Vector::from_array([
			EasyBigFloat::from_f64(10.0),
			EasyBigFloat::from_f64(10.0),
			EasyBigFloat::from_f64(10.0),
		]);
		let b = Vector::from_array([
			EasyBigFloat::from_f64(20.0),
			EasyBigFloat::from_f64(20.0),
			EasyBigFloat::from_f64(20.0),
		]);

		let c = Vector::from_array([
			EasyBigFloat::from_f64(30.0),
			EasyBigFloat::from_f64(30.0),
			EasyBigFloat::from_f64(30.0),
		]);

		assert_eq!(a, a);
		assert_eq!(c, a + b);
	}
}
