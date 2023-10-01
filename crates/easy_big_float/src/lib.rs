use std::{
	fmt::{Binary, Display, Octal, UpperHex},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
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

//const P: usize = 1024;
const RM: RoundingMode = RoundingMode::None;

lazy_static! {
	static ref CC: Mutex<Consts> = Mutex::new(Consts::new().expect("Couldn't create consts cache"));
}

#[derive(Debug)]
pub struct EasyBigFloat<const P: usize> {
	val: BigFloat,
}

impl<const P: usize> EasyBigFloat<P> {
	#[inline]
	pub fn new() -> Self {
		Self {
			val: BigFloat::new(P),
		}
	}
	#[inline]
	pub fn from_f64(f: f64) -> Self {
		Self {
			val: BigFloat::from_f64(f, P),
		}
	}
	#[inline]
	pub fn from_f32(f: f32) -> Self {
		Self {
			val: BigFloat::from_f32(f, P),
		}
	}
}

impl<const P: usize> Default for EasyBigFloat<P> {
	#[inline]
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
	($bound:ident,$fn:ident,$fn2:ident,$lhs:ty,$rhs:ty) => {
		impl<const P: usize> $bound<$rhs> for $lhs {
			type Output = EasyBigFloat<P>;
			#[inline]
			fn $fn(self, rhs: $rhs) -> Self::Output {
				Self::Output {
					val: self.val.$fn2(&rhs.val),
				}
			}
		}
	};
	($bound:ident,$fn:ident,$fn2:ident,$lhs:ty,$rhs:ty,$rm:ident) => {
		impl<const P: usize> $bound<$rhs> for $lhs {
			type Output = EasyBigFloat<P>;
			#[inline]
			fn $fn(self, rhs: $rhs) -> Self::Output {
				Self::Output {
					val: self.val.$fn2(&rhs.val, P, $rm),
				}
			}
		}
	};
}

macro_rules! impl_arith_assign {
	($bound:ident,$fn:ident,$fn2:ident,$lhs:ty,$rhs:ty) => {
		impl<const P: usize> $bound<$rhs> for $lhs {
			#[inline]
			fn $fn(&mut self, rhs: $rhs) {
				self.val = self.val.$fn2(&rhs.val);
			}
		}
	};
	($bound:ident,$fn:ident,$fn2:ident,$lhs:ty,$rhs:ty,$p:ident,$rm:ident) => {
		impl<const P: usize> $bound<$rhs> for $lhs {
			#[inline]
			fn $fn(&mut self, rhs: $rhs) {
				self.val = self.val.$fn2(&rhs.val, $p, $rm);
			}
		}
	};
}

macro_rules! impl_arith_4 {
	($bound:ident,$fn:ident,$tt:ty) => {
		impl_arith!($bound, $fn, $fn, $tt, $tt);
		impl_arith!($bound, $fn, $fn, &$tt, &$tt);
		impl_arith!($bound, $fn, $fn, $tt, &$tt);
		impl_arith!($bound, $fn, $fn, &$tt, $tt);
	};
	($bound:ident,$fn:ident,$tt:ty,$rm:ident) => {
		impl_arith!($bound, $fn, $fn, $tt, $tt, $rm);
		impl_arith!($bound, $fn, $fn, &$tt, &$tt, $rm);
		impl_arith!($bound, $fn, $fn, $tt, &$tt, $rm);
		impl_arith!($bound, $fn, $fn, &$tt, $tt, $rm);
	};
}

// ADD
//
impl_arith_4!(Add, add, EasyBigFloat<P>, RM);

// ADD ASSIGN
//
impl_arith_assign!(
	AddAssign,
	add_assign,
	add_full_prec,
	EasyBigFloat<P>,
	EasyBigFloat<P>
);
impl_arith_assign!(
	AddAssign,
	add_assign,
	add_full_prec,
	EasyBigFloat<P>,
	&EasyBigFloat<P>
);

// CLONE
//
impl<const P: usize> Clone for EasyBigFloat<P> {
	#[inline]
	fn clone(&self) -> Self {
		Self {
			val: self.val.clone(),
		}
	}
}

// DISPLAY
macro_rules! impl_display {
	($bound:ident) => {
		impl<const P: usize> $bound for EasyBigFloat<P> {
			#[inline]
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
impl_arith_4!(Div, div, EasyBigFloat<P>, RM);

// DIV ASSIGN
//
impl_arith_assign!(
	DivAssign,
	div_assign,
	div,
	EasyBigFloat<P>,
	EasyBigFloat<P>,
	P,
	RM
);
impl_arith_assign!(
	DivAssign,
	div_assign,
	div,
	EasyBigFloat<P>,
	&EasyBigFloat<P>,
	P,
	RM
);

// MUL
//
impl_arith_4!(Mul, mul, EasyBigFloat<P>, RM);

// MUL ASSIGN
//
impl_arith_assign!(
	MulAssign,
	mul_assign,
	mul_full_prec,
	EasyBigFloat<P>,
	EasyBigFloat<P>
);
impl_arith_assign!(
	MulAssign,
	mul_assign,
	mul_full_prec,
	EasyBigFloat<P>,
	&EasyBigFloat<P>
);

// NEG
//
impl<const P: usize> Neg for &EasyBigFloat<P> {
	type Output = EasyBigFloat<P>;
	#[inline]
	fn neg(self) -> Self::Output {
		Self::Output {
			val: self.val.clone().neg(),
		}
	}
}

impl<const P: usize> Neg for EasyBigFloat<P> {
	type Output = EasyBigFloat<P>;
	#[inline]
	fn neg(self) -> Self::Output {
		Self::Output {
			val: self.val.neg(),
		}
	}
}

// PARTIAL_EQ
//
impl<const P: usize> PartialEq for EasyBigFloat<P> {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		self.val == other.val
	}
}

impl<const P: usize> Eq for EasyBigFloat<P> {}

// PARTIAL_ORD
//
impl<const P: usize> PartialOrd for EasyBigFloat<P> {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.val.partial_cmp(&other.val)
	}
}

// REM
//
impl_arith_4!(Rem, rem, EasyBigFloat<P>);

// REM ASSIGN
//
impl_arith_assign!(RemAssign, rem_assign, rem, EasyBigFloat<P>, EasyBigFloat<P>);
impl_arith_assign!(
	RemAssign,
	rem_assign,
	rem,
	EasyBigFloat<P>,
	&EasyBigFloat<P>
);

// SUB
//
impl_arith_4!(Sub, sub, EasyBigFloat<P>, RM);

// SUB ASSIGN
//
impl_arith_assign!(
	SubAssign,
	sub_assign,
	sub_full_prec,
	EasyBigFloat<P>,
	EasyBigFloat<P>
);
impl_arith_assign!(
	SubAssign,
	sub_assign,
	sub_full_prec,
	EasyBigFloat<P>,
	&EasyBigFloat<P>
);

impl<const P: usize> Zero for EasyBigFloat<P> {
	#[inline]
	fn zero() -> Self {
		Self {
			val: BigFloat::from_f64(0.0, P),
		}
	}
	#[inline]
	fn is_zero(&self) -> bool {
		*self == EasyBigFloat::zero()
	}
}

impl<const P: usize> One for EasyBigFloat<P> {
	#[inline]
	fn one() -> Self {
		Self {
			val: BigFloat::from_f64(1.0, P),
		}
	}
}

impl<const P: usize> Num for EasyBigFloat<P> {
	type FromStrRadixErr = ();
	#[inline]
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

#[inline]
fn atan2<const P: usize>(y: &EasyBigFloat<P>, x: &EasyBigFloat<P>) -> EasyBigFloat<P> {
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

impl<const P: usize> EasyBigFloat<P> {
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
		atan2(self, other)
	}

	impl_fn!(powi, usize);
	impl_fn!(sqrt);
}

macro_rules! impl_trig {
	($bound:ident,$fn:ident) => {
		impl<const P: usize> $bound for EasyBigFloat<P> {
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

impl<const P: usize> Atan2 for EasyBigFloat<P> {
	#[inline]
	fn atan2(&self, other: &Self) -> Self {
		EasyBigFloat::atan2(other, self)
	}
}

impl<const P: usize> Pow for EasyBigFloat<P> {
	#[inline]
	fn pow(&self, n: &Self) -> Self {
		EasyBigFloat::pow(self, n)
	}

	#[inline]
	fn powi(&self, n: usize) -> Self {
		EasyBigFloat::powi(self, n)
	}
}

impl<const P: usize> SquareRoot for EasyBigFloat<P> {
	#[inline]
	fn sqrt(&self) -> Self {
		EasyBigFloat::sqrt(self)
	}
}

impl<const P: usize> From<f64> for EasyBigFloat<P> {
	#[inline]
	fn from(value: f64) -> Self {
		Self {
			val: BigFloat::from(value),
		}
	}
}

impl<const P: usize> From<f32> for EasyBigFloat<P> {
	#[inline]
	fn from(value: f32) -> Self {
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
		let a = EasyBigFloat::<1024>::from_f64(10.0);
		let b = EasyBigFloat::<1024>::from_f64(1.0);
		let c = EasyBigFloat::<1024>::from_f64(11.0);

		assert_eq!(c, &a + &b);
		assert_eq!(a, &c - &b);

		let one = EasyBigFloat::<1024>::one();
		let zero = EasyBigFloat::<1024>::zero();
		println!("{}", &one);
		dbg!(&one);
		println!("{}", &zero);
		dbg!(&zero);

		let one_zero = &one - &zero;
		println!("{}", &one_zero);
		dbg!(&one_zero);

		assert_eq!(one, one_zero);

		let one = BigFloat::from_f64(1.0, 1024);
		let zero = BigFloat::from_f64(0.0, 1024);
		println!("{}", &one);
		dbg!(&one);
		println!("{}", &zero);
		dbg!(&zero);

		let one_zero = one.sub(&zero, 1024, RoundingMode::None);
		println!("{}", &one_zero);
		dbg!(&one_zero);

		assert_eq!(one, one_zero);
	}

	#[test]
	fn test_vec() {
		let a = Vector::from_array([
			EasyBigFloat::<1024>::from_f64(10.0),
			EasyBigFloat::<1024>::from_f64(10.0),
			EasyBigFloat::<1024>::from_f64(10.0),
		]);
		let b = Vector::from_array([
			EasyBigFloat::<1024>::from_f64(20.0),
			EasyBigFloat::<1024>::from_f64(20.0),
			EasyBigFloat::<1024>::from_f64(20.0),
		]);

		let c = Vector::from_array([
			EasyBigFloat::<1024>::from_f64(30.0),
			EasyBigFloat::<1024>::from_f64(30.0),
			EasyBigFloat::<1024>::from_f64(30.0),
		]);

		assert_eq!(a, a);
		assert_eq!(c, a + b);
	}
}
