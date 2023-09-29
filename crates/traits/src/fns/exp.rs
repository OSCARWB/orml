pub trait ExpFns: Exp + Exp2 + ExpM1 {}

pub trait Exp {
	fn exp(self) -> Self;
}

pub trait Exp2 {
	fn exp2(self) -> Self;
}

pub trait ExpM1 {
	fn exp_m1(self) -> Self;
}
