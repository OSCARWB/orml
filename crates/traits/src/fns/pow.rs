pub trait Pow {
	fn pow(self, n: Self) -> Self;
}

pub trait Powi<T> {
	fn powi(self, n: T) -> Self;
}
