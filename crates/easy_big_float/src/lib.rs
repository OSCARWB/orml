use std::{
	fmt::{self, Binary, Display, Octal, UpperHex},
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
use traits::fns::SquareRoot;

const P: usize = 1024;
const RM: RoundingMode = RoundingMode::None;

lazy_static! {
	static ref CC: Mutex<Consts> = Mutex::new(Consts::new().expect("Couldn't create consts cache"));
}

#[derive(Debug)]
struct EasyBigFloat {
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
impl Deref for EasyBigFloat {
	type Target = BigFloat;

	fn deref(&self) -> &Self::Target {
		&self.val
	}
}

impl DerefMut for EasyBigFloat {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.val
	}
}

// ADD
//
impl Add<&EasyBigFloat> for &EasyBigFloat {
	type Output = EasyBigFloat;

	fn add(self, rhs: &EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.add_full_prec(&rhs.val),
		}
	}
}

impl Add<&EasyBigFloat> for EasyBigFloat {
	type Output = EasyBigFloat;

	fn add(self, rhs: &EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.add_full_prec(&rhs.val),
		}
	}
}

impl<'a> Add<EasyBigFloat> for &'a EasyBigFloat {
	type Output = EasyBigFloat;

	fn add(self, rhs: EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.add_full_prec(&rhs.val),
		}
	}
}

impl Add<EasyBigFloat> for EasyBigFloat {
	type Output = EasyBigFloat;

	fn add(self, rhs: EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.add_full_prec(&rhs.val),
		}
	}
}

// ADD ASSIGN
//
impl AddAssign<&EasyBigFloat> for EasyBigFloat {
	fn add_assign(&mut self, rhs: &EasyBigFloat) {
		self.val = self.val.add_full_prec(&rhs.val);
	}
}

impl AddAssign<EasyBigFloat> for EasyBigFloat {
	fn add_assign(&mut self, rhs: EasyBigFloat) {
		self.val = self.val.add_full_prec(&rhs.val);
	}
}

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
impl Display for EasyBigFloat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self, f)
	}
}
impl Binary for EasyBigFloat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Binary::fmt(&self, f)
	}
}
impl Octal for EasyBigFloat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Octal::fmt(&self, f)
	}
}
impl UpperHex for EasyBigFloat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		UpperHex::fmt(&self, f)
	}
}

// DIV
//
impl Div<&EasyBigFloat> for &EasyBigFloat {
	type Output = EasyBigFloat;

	fn div(self, rhs: &EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.div(&rhs.val, P, RM),
		}
	}
}

impl Div<&EasyBigFloat> for EasyBigFloat {
	type Output = EasyBigFloat;

	fn div(self, rhs: &EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.div(&rhs.val, P, RM),
		}
	}
}

impl<'a> Div<EasyBigFloat> for &'a EasyBigFloat {
	type Output = EasyBigFloat;

	fn div(self, rhs: EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.div(&rhs.val, P, RM),
		}
	}
}

impl Div<EasyBigFloat> for EasyBigFloat {
	type Output = EasyBigFloat;

	fn div(self, rhs: EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.div(&rhs.val, P, RM),
		}
	}
}

// DIV ASSIGN
impl DivAssign<&EasyBigFloat> for EasyBigFloat {
	fn div_assign(&mut self, rhs: &EasyBigFloat) {
		self.val = self.val.div(&rhs.val, P, RM);
	}
}

impl DivAssign<EasyBigFloat> for EasyBigFloat {
	fn div_assign(&mut self, rhs: EasyBigFloat) {
		self.val = self.val.div(&rhs.val, P, RM);
	}
}

// MUL
//
impl Mul<&EasyBigFloat> for &EasyBigFloat {
	type Output = EasyBigFloat;

	fn mul(self, rhs: &EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.mul_full_prec(&rhs.val),
		}
	}
}

impl Mul<&EasyBigFloat> for EasyBigFloat {
	type Output = EasyBigFloat;

	fn mul(self, rhs: &EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.mul_full_prec(&rhs.val),
		}
	}
}

impl<'a> Mul<EasyBigFloat> for &'a EasyBigFloat {
	type Output = EasyBigFloat;

	fn mul(self, rhs: EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.mul_full_prec(&rhs.val),
		}
	}
}

impl Mul<EasyBigFloat> for EasyBigFloat {
	type Output = EasyBigFloat;

	fn mul(self, rhs: EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.mul_full_prec(&rhs.val),
		}
	}
}

// MUL ASSIGN
//
impl MulAssign<EasyBigFloat> for EasyBigFloat {
	fn mul_assign(&mut self, rhs: EasyBigFloat) {
		self.val = self.val.mul_full_prec(&rhs.val);
	}
}

impl MulAssign<&EasyBigFloat> for EasyBigFloat {
	fn mul_assign(&mut self, rhs: &EasyBigFloat) {
		self.val = self.val.mul_full_prec(&rhs.val);
	}
}

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
impl Rem<&EasyBigFloat> for &EasyBigFloat {
	type Output = EasyBigFloat;

	fn rem(self, rhs: &EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.rem(&rhs.val),
		}
	}
}

impl Rem<&EasyBigFloat> for EasyBigFloat {
	type Output = EasyBigFloat;

	fn rem(self, rhs: &EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.rem(&rhs.val),
		}
	}
}

impl<'a> Rem<EasyBigFloat> for &'a EasyBigFloat {
	type Output = EasyBigFloat;

	fn rem(self, rhs: EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.rem(&rhs.val),
		}
	}
}

impl Rem<EasyBigFloat> for EasyBigFloat {
	type Output = EasyBigFloat;

	fn rem(self, rhs: EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.rem(&rhs.val),
		}
	}
}

// REM ASSIGN
//
impl RemAssign<&EasyBigFloat> for EasyBigFloat {
	fn rem_assign(&mut self, rhs: &EasyBigFloat) {
		self.val = self.val.rem(&rhs.val);
	}
}

impl RemAssign<EasyBigFloat> for EasyBigFloat {
	fn rem_assign(&mut self, rhs: EasyBigFloat) {
		self.val = self.val.rem(&rhs.val);
	}
}

// SUB
//
impl Sub<&EasyBigFloat> for &EasyBigFloat {
	type Output = EasyBigFloat;

	fn sub(self, rhs: &EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.sub_full_prec(&rhs.val),
		}
	}
}

impl Sub<&EasyBigFloat> for EasyBigFloat {
	type Output = EasyBigFloat;

	fn sub(self, rhs: &EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.sub_full_prec(&rhs.val),
		}
	}
}

impl<'a> Sub<EasyBigFloat> for &'a EasyBigFloat {
	type Output = EasyBigFloat;

	fn sub(self, rhs: EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.sub_full_prec(&rhs.val),
		}
	}
}

impl Sub<EasyBigFloat> for EasyBigFloat {
	type Output = EasyBigFloat;

	fn sub(self, rhs: EasyBigFloat) -> Self::Output {
		Self::Output {
			val: self.val.sub_full_prec(&rhs.val),
		}
	}
}

// SUB ASSIGN
//
impl SubAssign<EasyBigFloat> for EasyBigFloat {
	fn sub_assign(&mut self, rhs: EasyBigFloat) {
		self.val = self.val.sub_full_prec(&rhs.val);
	}
}

impl SubAssign<&EasyBigFloat> for EasyBigFloat {
	fn sub_assign(&mut self, rhs: &EasyBigFloat) {
		self.val = self.val.sub_full_prec(&rhs.val);
	}
}

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

impl SquareRoot for EasyBigFloat {
	fn sqrt(&self) -> Self {
		Self {
			val: BigFloat::sqrt(self, P, RM),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let a = EasyBigFloat::from_f64(10.0);
		let b = EasyBigFloat::from_f64(1.0);
		let c = EasyBigFloat::from_f64(11.0);

		assert_eq!(c, a + b);
	}
}
