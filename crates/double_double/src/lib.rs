use std::ops::{Add, Sub};

pub struct DoubleDouble(f64, f64);

impl From<(f64, f64)> for DoubleDouble {
	fn from(value: (f64, f64)) -> Self {
		Self(value.0, value.1)
	}
}

impl From<DoubleDouble> for (f64, f64) {
	fn from(value: DoubleDouble) -> Self {
		(value.0, value.1)
	}
}

impl Add for DoubleDouble {
	type Output = DoubleDouble;
	fn add(self, rhs: Self) -> Self::Output {
		add2(self.into(), rhs.into()).into()
	}
}

impl Sub for DoubleDouble {
	type Output = DoubleDouble;
	fn sub(self, rhs: Self) -> Self::Output {
		sub2(self.into(), rhs.into()).into()
	}
}

fn zzz(r: f64, s: f64) -> (f64, f64) {
	let z = r + s;
	let zz = r - z + s;
	(z, zz)
}

pub fn add2(x: (f64, f64), y: (f64, f64)) -> (f64, f64) {
	let r = x.0 + y.0;
	let s = if x.0.abs() > y.0.abs() {
		x.0 - r + y.0 + y.1 + x.1
	} else {
		y.0 - r + x.0 + x.1 + y.1
	};
	zzz(r, s)
}

pub fn sub2(x: (f64, f64), y: (f64, f64)) -> (f64, f64) {
	let r = x.0 - y.0;
	let s = if x.0.abs() > y.0.abs() {
		x.0 - r - y.0 - y.1 + x.1
	} else {
		-y.0 - r + x.0 + x.1 - y.1
	};
	zzz(r, s)
}

#[allow(non_snake_case)]
pub fn mul12(x: f64, y: f64) -> (f64, f64) {
	let A = split(x);
	let B = split(y);

	let p = A.0 * B.0;
	let q = A.0 * B.1 + A.1 * B.0;

	let r = p + q;
	(r, p - r + q + A.1 * B.0)
}

pub fn mul(x: (f64, f64), y: (f64, f64)) -> (f64, f64) {
	let t = mul12(x.0, y.0);

	let c = x.0 * y.1 + x.1 * y.1 + t.1;

	zzz(t.0, c)
}

pub fn div(x: (f64, f64), y: (f64, f64)) -> (f64, f64) {
	let u = x.0 / y.0;

	let t = mul12(u, y.0);

	let l = (x.0 - t.0 - t.1 + x.1 - u * y.1) / y.0;
	zzz(u, l)
}

pub fn sqrt(x: (f64, f64)) -> (f64, f64) {
	if x.0 > 0.0 {
		let c = x.0.sqrt();
		let u = mul12(c, c);
		let cc = (x.0 - u.0 - u.1 + x.1) * 0.5 / c;
		zzz(c, cc)
	} else {
		(0.0, 0.0)
	}
}

const SCALE: f64 = (2usize.pow(53 / 2) + 1) as f64;

pub fn split(a: f64) -> (f64, f64) {
	let t = a * SCALE;
	let hi = a - t + t;
	(hi, a - hi)
}

pub fn quick_two_sum(a: f64, b: f64) -> (f64, f64) {
	let s = a + b;
	let e = b - (s - a);
	(s, e)
}

pub fn two_sum(a: f64, b: f64) -> (f64, f64) {
	let s = a + b;
	let v = s - a;
	let e = (a - (s - v)) + (b - v);
	(s, e)
}

pub fn two_prod(a: f64, b: f64) -> (f64, f64) {
	let p = a * b;
	let a = split(a);
	let b = split(b);
	let e = ((a.0 * b.0 - p) + a.0 * b.1 + a.1 * b.0) + a.1 * b.1;
	(p, e)
}

pub fn two_prod_fma(a: f64, b: f64) -> (f64, f64) {
	let p = a * b;
	let e = a * b - p;
	(p, e)
}

pub fn renormalise(a: (f64, f64, f64, f64, f64)) -> (f64, f64, f64, f64) {
	let mut t: [f64; 5] = Default::default();

	let x = quick_two_sum(a.3, a.4);
	t[4] = x.1;
	let x = quick_two_sum(a.2, x.0);
	t[3] = x.1;
	let x = quick_two_sum(a.2, x.0);
	t[2] = x.1;
	let x = quick_two_sum(a.0, x.0);
	t[1] = x.1;
	t[0] = x.0;

	let mut s = t[0];
	let mut k = 0;
	let mut b: [f64; 4] = Default::default();
	for i in 1..=4 {
		let x = quick_two_sum(s, t[i]);
		if x.1 != 0.0 {
			b[k] = x.0;
			s = x.1;
			k += 1;
		}
	}

	(b[0], b[1], b[2], b[3])
}
