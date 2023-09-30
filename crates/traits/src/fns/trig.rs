pub trait TrigFns:
	Sin + Sinh + SinCos + Asin + Asinh + Cos + Cosh + Acos + Acosh + Tan + Tanh + Atan + Atan2 + Atanh
{
}

#[macro_export]
macro_rules! impl_all_trig {
	($impl_trig:ident) => {
		impl_trig!(Acos, acos);
		impl_trig!(Acosh, acosh);
		impl_trig!(Asin, asin);
		impl_trig!(Asinh, asinh);
		impl_trig!(Atan, atan);
		impl_trig!(Atanh, atanh);
		impl_trig!(Cos, cos);
		impl_trig!(Cosh, cosh);
		impl_trig!(Sin, sin);
		impl_trig!(Sinh, sinh);
		impl_trig!(Tan, tan);
		impl_trig!(Tanh, tanh);
	};
}

#[macro_export]
macro_rules! impl_trivial_trig {
	($bound:ident,$fn:ident,$tt:ty) => {
		impl $bound for $tt {
			fn $fn(&self) -> Self {
				<$tt>::$fn(*self)
			}
		}
	};
}

#[macro_export]
macro_rules! impl_trivial_trig_all {
	($tt:ty) => {
		impl_trivial_trig!(Acos, acos, $tt);
		impl_trivial_trig!(Acosh, acosh, $tt);
		impl_trivial_trig!(Asin, asin, $tt);
		impl_trivial_trig!(Asinh, asinh, $tt);
		impl_trivial_trig!(Atan, atan, $tt);
		impl_trivial_trig!(Atanh, atanh, $tt);
		impl_trivial_trig!(Cos, cos, $tt);
		impl_trivial_trig!(Cosh, cosh, $tt);
		impl_trivial_trig!(Sin, sin, $tt);
		impl_trivial_trig!(Sinh, sinh, $tt);
		impl_trivial_trig!(Tan, tan, $tt);
		impl_trivial_trig!(Tanh, tanh, $tt);
	};
}

pub trait Acos {
	fn acos(&self) -> Self;
}

pub trait Acosh {
	fn acosh(&self) -> Self;
}

pub trait Asin {
	fn asin(&self) -> Self;
}
pub trait Asinh {
	fn asinh(&self) -> Self;
}

pub trait Atan {
	fn atan(&self) -> Self;
}

pub trait Atan2 {
	fn atan2(&self, other: &Self) -> Self;
}

pub trait Atanh {
	fn atanh(&self) -> Self;
}
pub trait Cos {
	fn cos(&self) -> Self;
}

pub trait Cosh {
	fn cosh(&self) -> Self;
}

pub trait Sin {
	fn sin(&self) -> Self;
}

pub trait SinCos: std::marker::Sized {
	fn sin_cos(&self) -> (Self, Self);
}

pub trait Sinh {
	fn sinh(&self) -> Self;
}

pub trait Tan {
	fn tan(&self) -> Self;
}
pub trait Tanh {
	fn tanh(&self) -> Self;
}
