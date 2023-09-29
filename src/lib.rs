#![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
#![allow(dead_code)]

//! ORML: Oscar's Rust Math Library
//! A library full of maths, including such delights as:
//! - fractions
//! - vectors
//! - quaternions
//! - easy big floats
//! - and many more to come

//#[cfg(feature = "vector")]
pub use orml_vector as vector;

//#[cfg(feature = "traits")]
pub use orml_traits as traits;

pub mod fraction;

//#[cfg(feature = "quaternion")]
pub use orml_quaternion as quaternion;

pub use orml_easy_big_float as easy_big_float;
