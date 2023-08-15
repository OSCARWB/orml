#![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
#![allow(dead_code)]

//! ORML: Oscar's Rust Math Library
//! A library full of maths, including such delight as:
//! - fractions
//! - vectors
//! - and many more to come

#[cfg(feature = "vector")]
pub use ::vector::Vector;

#[cfg(feature = "traits")]
pub use traits;

pub mod fraction;

#[cfg(feature = "quaternion")]
pub use ::quaternion;
