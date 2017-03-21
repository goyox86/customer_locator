// Copyright 2017 Jose Narvaez. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

/// Struct representing a location on earth surface.
///
/// It's is main responsibility is to hold state about
/// latitude and longitude and also act as a place holder
/// for the calculations like the distance between two
/// `Location` instances.
///
/// # Examples
///
/// You can explicitly create a [`Location`] with [`new`]:
///
/// ```
/// let latitude = 53.3393;
/// let longitude = -6.2576841;
///
/// let location: Location::new(latitude, longitude);
/// assert_eq!(location, Location { latitude: 53.3393, longitude: -6.2576841});
/// ```
///
use std::str::FromStr;
use std::convert::From;
use std::num::ParseFloatError;
use std::error;
use std::fmt;

use units::Kilometers;

const EARTH_RADIUS_IN_KM: f64 = 6371f64;
const DUBLIN_LAT: f64 = 53.3393;
const DUBLIN_LONG: f64 = -6.2576841;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl Location {
    /// Constructs a new `Location` given the `latitude` and `longitude`.
    ///
    /// # Examples
    ///
    /// You can explicitly create a [`Location`] with [`new`]:
    ///
    /// ```
    /// let latitude = 53.3393;
    /// let longitude = -6.2576841;
    ///
    /// let location: Location::new(latitude, longitude);
    /// assert_eq!(location, Location { latitude: 53.3393, longitude: -6.2576841})
    /// ```
    pub fn new(latitude: f64, longitude: f64) -> Location {
        Location {
            latitude: latitude,
            longitude: longitude,
        }
    }

    /// Returns a the distance in `Kilometers` between the `self` and other
    /// `Location` and the provided one.
    ///
    /// This method calculates the distance in `Kilometers` between `self`
    /// and `other ` on earth surface using the [Harversine formula]
    /// (https://en.wikipedia.org/wiki/Haversine_formula)
    ///
    /// # Examples
    ///
    /// ```
    /// let location: Location::new(53.3393, -6.2576841); // Dublin
    ///
    /// let other_location = Location::new(-6.238335, 53.2451022); // 10 Km outside Dublin
    /// assert_eq!(location.distance_from(other_location), Kilometers(10.553));
    /// ```
    pub fn distance_from(&self, other: &Location) -> Kilometers {
        let latitude_1 = self.latitude.to_radians();
        let latitude_2 = other.latitude.to_radians();
        let delta_latitude = (other.latitude - self.latitude).to_radians();
        let delta_longitude = (other.longitude - self.longitude).to_radians();

        // not very self-descriptive name just used as a placeholder to improve readability
        // and they refer to the names given to the sides of the triangle on the surface
        // referenced in the "Law of haversines".
        let a = (delta_latitude / 2.0f64).sin() * (delta_latitude / 2.0f64).sin() +
                latitude_1.cos() * latitude_2.cos() * (delta_longitude / 2.0f64).sin() *
                (delta_longitude / 2.0f64);

        let c = 2.0f64 * a.sqrt().atan2((1.0f64 - a).sqrt());

        Kilometers(EARTH_RADIUS_IN_KM * c)
    }

    /// Returns a whether the location is Dublin, Ireland.
    ///
    /// Just a convenience function used for the purposes of the exercise.
    ///
    /// # Examples
    ///
    /// ```
    /// let dublin: Location::new(53.3393, -6.2576841); // Dublin
    /// assert_eq!(dublin.is_dublin(), true);
    /// 
    /// let new_york = Location(40.7128, 74.0059);
    /// assert_eq!(new_york.is_dublin(), false);
    /// ```
    pub fn is_dublin(&self) -> bool {
        *self == Self::dublin()
    }

    /// Constructs a new `Location` object with the coordinates of Dublin, Ireland.
    ///
    /// Just a convenience function used for the purposes of the exercise.
    ///
    /// # Examples
    ///
    /// ```
    /// let dublin: Location::dublin(); // Dublin
    /// assert_eq!(dublin.is_dublin(), true);
    /// ```
    pub fn dublin() -> Location {
        Location::new(DUBLIN_LAT, DUBLIN_LONG)
    }
}

/// An error when trying to build a `Location` instance from a `&str`.
///
/// Akin to Parse*Error from Rust standard library. Used on the
/// implementation of `std::str::FromStr`
///
#[derive(Debug)]
pub struct ParseLocationError(String);

impl From<ParseFloatError> for ParseLocationError {
    fn from(parse_err: ParseFloatError) -> Self {
        ParseLocationError(format!("error parsing location {}", parse_err))
    }
}

impl FromStr for Location {
    type Err = ParseLocationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<&str> = s.split(",").collect();
        if coordinates.len() < 2 {
            return Err(ParseLocationError(String::from("missing element latitude,longitude on tuple")))
        }
        let latitude = f64::from_str(coordinates[0])?;
        let longitude = f64::from_str(coordinates[1])?;

        Ok(Location::new(latitude, longitude))
    }
}

impl fmt::Display for ParseLocationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Location parse error: {}", self.0)
    }
}

impl error::Error for ParseLocationError {
    fn description(&self) -> &str {
        &self.0
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(self)
    }
}

impl fmt::Display for Location {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "Location({}, {})", self.latitude, self.longitude)
     }
}