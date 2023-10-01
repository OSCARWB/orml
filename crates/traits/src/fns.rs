pub mod trig;

pub mod float;

pub mod log;

pub mod exp;

pub mod abs;

pub mod pow;

pub trait CubeRoot {
	fn cbrt(self) -> Self;
}

pub trait Ceil {
	fn ceil(self) -> Self;
}

pub trait Clamp {
	fn clamp(self, min: Self, max: Self) -> Self;
}

pub trait DivEuclid {
	fn div_euclid(self, rhs: Self) -> Self;
}

pub trait Floor {
	fn floor(self) -> Self;
}

pub trait Fract {
	fn fract(self) -> Self;
}

pub trait Hypot {
	fn hypot(self, other: Self) -> Self;
}

pub trait Signed {
	fn is_sign_positive(&self) -> bool;
	fn is_sign_negative(&self) -> bool;
	fn signum(&self) -> Self;
	fn copysign(&self, sign: &Self) -> Self;
}

pub trait Max {
	fn max(self, other: Self) -> Self;
}

pub trait Min {
	fn min(self, other: Self) -> Self;
}

pub trait MulAdd {
	fn mul_add(self, a: Self, b: Self) -> Self;
}

pub trait Recip {
	fn recip(self) -> Self;
}

pub trait RemEuclid {
	fn rem_euclid(self, rhs: Self) -> Self;
}

pub trait Round {
	fn round(self) -> Self;
}

pub trait SquareRoot {
	fn sqrt(&self) -> Self;
}

pub trait Degrees {
	fn to_degrees(self) -> Self;
	fn to_radians(self) -> Self;
}

pub trait Trunc {
	fn trunc(self) -> Self;
}
