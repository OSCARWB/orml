use std::ops;

#[derive(Debug, PartialEq)]
struct Fraction<T> {
	numerator: T,
	denominator: T,
}

impl<T> ops::Mul for Fraction<T>
where
	T: ops::Mul<Output = T>,
{
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self {
			numerator: self.numerator * rhs.numerator,
			denominator: self.denominator * rhs.denominator,
		}
	}
}

impl<T> ops::Mul<T> for Fraction<T>
where
	T: ops::Mul<Output = T>,
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self::Output {
		Self {
			numerator: self.numerator * rhs,
			denominator: self.denominator,
		}
	}
}

impl<T> From<(T, T)> for Fraction<T> {
	fn from(value: (T, T)) -> Self {
		Self {
			numerator: value.0,
			denominator: value.1,
		}
	}
}

struct F<T>(T);
impl<T> From<Fraction<T>> for F<T>
where
	T: ops::Div<Output = F<T>>,
{
	fn from(value: Fraction<T>) -> F<T> {
		value.numerator / value.denominator
	}
}

#[cfg(test)]
mod tests {
	#![allow(non_camel_case_types)]

	use super::*;
	type iFrac64 = Fraction<i64>;

	//#[test]
	fn test_mul() {
		let num = 10_000; //3037000164;

		for n in -num..num {
			for d in -num..num {
				let frac1: iFrac64 = (n, d).into();
				let frac2: iFrac64 = (d, n).into();
				let frac3 = frac1 * frac2;

				assert_eq!(frac3, (n * d, n * d).into());
			}
		}
	}
}
