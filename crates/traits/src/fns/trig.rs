pub trait TrigFns:
	Sin + Sinh + SinCos + Asin + Asinh + Cos + Cosh + Acos + Acosh + Tan + Tanh + Atan + Atan2 + Atanh
{
}

pub trait Acos {
	fn acos(self) -> Self;
}

pub trait Acosh {
	fn acosh(self) -> Self;
}

pub trait Asin {
	fn asin(self) -> Self;
}
pub trait Asinh {
	fn asinh(self) -> Self;
}

pub trait Atan {
	fn atan(self) -> Self;
}

pub trait Atan2 {
	fn atan2(self, other: Self) -> Self;
}

pub trait Atanh {
	fn atanh(self) -> Self;
}
pub trait Cos {
	fn cos(self) -> Self;
}

pub trait Cosh {
	fn cosh(self) -> Self;
}

pub trait Sin {
	fn sin(self) -> Self;
}

pub trait SinCos: std::marker::Sized {
	fn sin_cos(self) -> (Self, Self);
}

pub trait Sinh {
	fn sinh(self) -> Self;
}

pub trait Tan {
	fn tan(self) -> Self;
}
pub trait Tanh {
	fn tanh(self) -> Self;
}
