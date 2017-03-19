use std::fmt;

const EARTH_RADIUS_IN_KM: f64 = 6371f64;

#[derive(PartialEq, PartialOrd)]
pub struct Kilometers(pub f64);

impl fmt::Display for Kilometers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*} Km", 3, self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64
}

impl Location {
    pub fn new(latitude: f64, longitude: f64) -> Location {
        Location { latitude: latitude, longitude }
    }

    pub fn distance_from(&self, other: &Location) -> Kilometers {
        let phi_1 = self.latitude.to_radians();
        let phi_2 = other.latitude.to_radians();
        let delta_phi = (other.latitude - self.latitude).to_radians();
        let delta_lambda = (other.longitude - self.longitude).to_radians();

        let a = (delta_phi / 2.0f64).sin() *
               (delta_phi / 2.0f64).sin() +
               phi_1.cos() *
               phi_2.cos() *
               (delta_lambda / 2.0f64).sin() *
               (delta_lambda / 2.0f64);

        let c = 2.0f64 * a.sqrt().atan2((1.0f64 - a).sqrt());
        
        Kilometers(EARTH_RADIUS_IN_KM * c)
    }
}
