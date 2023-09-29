pub trait AbsFns: Abs + AbsSub {}

pub trait Abs {
	fn abs(self) -> Self;
}

pub trait AbsSub {
	fn abs_sub(self) -> Self;
}
