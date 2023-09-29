use std::num::FpCategory;

pub trait FloatFns: Infinite + Nan + Normal + Classify {}

pub trait Infinite {
	fn is_finite(&self) -> bool;
	fn is_infinite(&self) -> bool;
}

pub trait Nan {
	fn is_nan(&self) -> bool;
}

pub trait Normal {
	fn is_normal(&self) -> bool;
	fn is_subnormal(&self) -> bool;
}

pub trait Classify {
	fn classify(self) -> FpCategory;
}
