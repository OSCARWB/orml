pub trait Pow {
	fn pow(self, n: &Self) -> Self;
	fn powi(self, n: usize) -> Self;
}

pub trait Powi {
	fn powi(self, n: usize) -> Self;
}
