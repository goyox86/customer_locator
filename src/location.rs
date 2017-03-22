// Copyright 2017 Jose Narvaez. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::FromStr;
use std::convert::From;
use std::num::ParseFloatError;
use std::error;
use std::fmt;

use units::Kilometers;

const EARTH_RADIUS_IN_KM: f64 = 6372.8f64;
const DUBLIN_LAT: f64 = 53.3393;
const DUBLIN_LONG: f64 = -6.2576841;

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
    /// and `other` on earth surface using the [Harversine formula]
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
        let delta_latitude = (other.latitude - self.latitude).to_radians();
        let delta_longitude = (other.longitude - self.longitude).to_radians();
        let latitude_1 = self.latitude.to_radians();
        let latitude_2 = other.latitude.to_radians();

        // not very self-descriptive name just used as a placeholder to improve readability
        // and they refer to the names given to the sides of the triangle on the surface
        // referenced in the "Law of haversines".
        let a = ((delta_latitude / 2.0f64).sin().powf(2.0f64)) +
                latitude_1.cos() *
                latitude_2.cos() *
                ((delta_longitude / 2.0f64).sin().powf(2.0f64));

        let c = 2.0f64 * (a.sqrt().asin());

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
#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    const NY_LAT: f64 = 40.7128f64;
    const NY_LONG: f64 = -74.0059f64;
    const DUB_NY_DIST_IN_KM: Kilometers = Kilometers(5116.751541958293);

    #[test]
    fn new_builds_correct_instance() {
        let location = Location::new(NY_LAT, NY_LONG);
        assert_eq!(location.latitude, NY_LAT);
        assert_eq!(location.longitude, NY_LONG);
    }

    #[test]
    fn distance_from_calculates_distance_between_diff_points_long() {
        let dublin = Location::dublin();
        let new_york = Location::new(NY_LAT, NY_LONG);
        assert_eq!(dublin.distance_from(&new_york), DUB_NY_DIST_IN_KM);
        assert_eq!(new_york.distance_from(&dublin), DUB_NY_DIST_IN_KM);
    }

    #[test]
    fn distance_from_calculates_distance_same_point() {
        let dublin = Location::dublin();
        assert_eq!(dublin.distance_from(&dublin), Kilometers(0f64));
    }

    #[test]
    fn is_dublin_is_true_for_dublin() {
        let dublin = Location::dublin();
        assert_eq!(dublin.is_dublin(), true);
    }

    #[test]
    fn is_dublin_is_false_for_something_else_than_dublin() {
        let new_york = Location::new(NY_LAT, NY_LONG);
        assert_eq!(new_york.is_dublin(), false);
    }

    #[test]
    fn dublin_is_returns_an_instance_centered_at_dublin() {
        let expected_dublin = Location::new(DUBLIN_LAT, DUBLIN_LONG);
        assert_eq!(expected_dublin, Location::dublin());
    }

    #[test]
    fn from_str_returns_an_instance_centered_at_given_coordinates() {
        let location_str = format!("{},{}", DUBLIN_LAT, DUBLIN_LONG);
        let location_instance = Location::from_str(&location_str).expect("this location str was supposed to parse succesfully.");
        assert_eq!(location_instance, Location::dublin());
    }

    #[test]
    fn from_str_fails_with_missing_coord_when_only_one_coord() {
        let location_str = format!("{}", DUBLIN_LAT);
        let expected_error = ParseLocationError(String::from("missing element latitude,longitude on tuple"));
        let actual_error = Location::from_str(&location_str).unwrap_err();
        assert_eq!(expected_error, actual_error);
    }

    #[test]
    fn from_str_fails_with_float_point_parse_err_only_lat_and_comma() {
        let location_str = format!("{},", DUBLIN_LAT);
        let expected_error = ParseLocationError(String::from("error parsing location cannot parse float from empty string"));
        let actual_error = Location::from_str(&location_str).unwrap_err();
        assert_eq!(expected_error, actual_error);
    }

    #[test]
    fn from_str_fails_with_float_point_parse_err_only_long_and_comma() {
        let location_str = format!(",{}", DUBLIN_LONG);
        let expected_error = ParseLocationError(String::from("error parsing location cannot parse float from empty string"));
        let actual_error = Location::from_str(&location_str).unwrap_err();
        assert_eq!(expected_error, actual_error);
    }

    #[test]
    fn from_str_fails_with_float_point_parse_err_when_lat_is_not_a_float() {
        let location_str = "40.7128xxx,-74.0059";
        let expected_error = ParseLocationError(String::from("error parsing location invalid float literal"));
        let actual_error = Location::from_str(&location_str).unwrap_err();
        assert_eq!(expected_error, actual_error);
    }

    #[test]
    fn from_str_fails_with_float_point_parse_err_when_long_is_not_a_float() {
        let location_str = "40.7128,-74.0059asdf";
        let expected_error = ParseLocationError(String::from("error parsing location invalid float literal"));
        let actual_error = Location::from_str(&location_str).unwrap_err();
        assert_eq!(expected_error, actual_error);
    }

    #[test]
    fn from_str_fails_with_missing_coord_err_invalid_sep() {
        let location_str = "40.7128/-74.0059";
        let expected_error = ParseLocationError(String::from("missing element latitude,longitude on tuple"));
        let actual_error = Location::from_str(&location_str).unwrap_err();
        assert_eq!(expected_error, actual_error);
    }
}