// Copyright 2017 Jose Narvaez. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

//!
//! Module containing all the units used accross all the calculations
//! on the library. It is meant to hold a set of "NewTypes"" that provide
//! compiler aided support for checking inputs and outpus from calculation
//! functions and methods such as the `Kilometers` tuple struct.
//!
//! # Examples
//!
//! You can explicitly create a [`Kilometers`] with a literal
//! by wrapping an 64 bits floating point number:
//!
//! ```
//! let kms = Kilometers(10f64);
//!````
//!

use std::fmt;

/// Struct representing a distance in Kilometers.
///
/// It's is a "NewType" whose main responisibility is to
/// enable compiler backed checks for inputs and outputs
/// in methods and functions far superior than receiving
//  and returning bare floating point numbers that can be
/// interpreted in different ways. Imagine on function
/// assuming you are passing a distance in meters but
/// the calculation that originated the value was assuming
/// Kilometers
///
/// # Examples
///
/// You can explicitly create a [`Kilometers`] with a literal
/// by wrapping an 64 bits floating point number:
///
/// ```
/// let kilometers = Kilometers(10f64);
/// ```
///
#[derive(PartialEq, PartialOrd)]
pub struct Kilometers(pub f64);

impl fmt::Display for Kilometers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*} Km", 3, self.0)
    }
}
