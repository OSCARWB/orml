use easy_big_float::EasyBigFloat;
use vector::Vector;

fn main() {
	let a = Vector::from_array([
		EasyBigFloat::from_f64(10.0),
		EasyBigFloat::from_f64(10.0),
		EasyBigFloat::from_f64(10.0),
	]);
	let b = Vector::from_array([
		EasyBigFloat::from_f64(20.0),
		EasyBigFloat::from_f64(20.0),
		EasyBigFloat::from_f64(20.0),
	]);

	let c = Vector::from_array([
		EasyBigFloat::from_f64(30.0),
		EasyBigFloat::from_f64(30.0),
		EasyBigFloat::from_f64(30.0),
	]);

	assert_eq!(c, &a * &b);
	let d = a + b;
	for e in d.iter() {
		println!("{}", e);
	}
}
