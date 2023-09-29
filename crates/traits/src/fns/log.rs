pub trait LogFns: Ln + Ln1P + Log + Log10 + Log2 {}

pub trait Ln {
	fn ln(self) -> Self;
}

pub trait Ln1P {
	fn ln_1p(self) -> Self;
}

pub trait Log {
	fn log(self, base: Self) -> Self;
}

pub trait Log10 {
	fn log10(self, base: Self) -> Self;
}

pub trait Log2 {
	fn log2(self, base: Self) -> Self;
}
