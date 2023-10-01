use num_traits::{One, Zero};
use orml_easy_big_float::EasyBigFloat;
use orml_quaternion::Quaternion;
use orml_vector::Vector;

fn main() {
	let a = Vector::from_array([
		EasyBigFloat::<1024>::from_f64(10.0),
		EasyBigFloat::<1024>::from_f64(10.0),
		EasyBigFloat::<1024>::from_f64(10.0),
	]);
	let b = Vector::from_array([
		EasyBigFloat::<1024>::from_f64(20.0),
		EasyBigFloat::<1024>::from_f64(20.0),
		EasyBigFloat::<1024>::from_f64(20.0),
	]);

	let c = Vector::from_array([
		EasyBigFloat::<1024>::from_f64(30.0),
		EasyBigFloat::<1024>::from_f64(30.0),
		EasyBigFloat::<1024>::from_f64(30.0),
	]);

	assert_eq!(c, &a + &b);
	let d = &a + &b;
	for e in d.iter() {
		println!("{}", e);
	}

	let _axis = Vector::from_array([
		EasyBigFloat::<1024>::one(),
		EasyBigFloat::<1024>::zero(),
		EasyBigFloat::<1024>::zero(),
	]);

	//let q = Quaternion::from_axis_angle(axis, EasyBigFloat::from_f64(1.0));
	let q = Quaternion::from_axis_angle(
		[
			EasyBigFloat::<1024>::from_f64(1.0),
			EasyBigFloat::<1024>::from_f64(0.0),
			EasyBigFloat::<1024>::from_f64(0.0),
		],
		EasyBigFloat::<1024>::from_f64(1.0),
	);

	let _z = q * a;
}
