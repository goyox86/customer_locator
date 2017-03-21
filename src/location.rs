use std::str::FromStr;
use std::convert::From;
use std::num::ParseFloatError;
use std::error;
use std::fmt;

use units::Kilometers;

const EARTH_RADIUS_IN_KM: f64 = 6371f64;

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl Location {
    pub fn new(latitude: f64, longitude: f64) -> Location {
        Location {
            latitude: latitude,
            longitude: longitude,
        }
    }

    pub fn distance_from(&self, other: &Location) -> Kilometers {
        let latitude_1 = self.latitude.to_radians();
        let latitude_2 = other.latitude.to_radians();
        let delta_latitude = (other.latitude - self.latitude).to_radians();
        let delta_longitude = (other.longitude - self.longitude).to_radians();

        let a = (delta_latitude / 2.0f64).sin() * (delta_latitude / 2.0f64).sin() +
                latitude_1.cos() * latitude_2.cos() * (delta_longitude / 2.0f64).sin() *
                (delta_longitude / 2.0f64);

        let c = 2.0f64 * a.sqrt().atan2((1.0f64 - a).sqrt());

        Kilometers(EARTH_RADIUS_IN_KM * c)
    }
}

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