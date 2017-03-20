use units::Kilometers;

const EARTH_RADIUS_IN_KM: f64 = 6371f64;

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
        let latitude_1 = self.latitude.to_radians();
        let latitude_2 = other.latitude.to_radians();
        let delta_latitude = (other.latitude - self.latitude).to_radians();
        let delta_longitude = (other.longitude - self.longitude).to_radians();

        let a = (delta_latitude / 2.0f64).sin() *
                (delta_latitude / 2.0f64).sin() +
                latitude_1.cos() *
                latitude_2.cos() *
                (delta_longitude / 2.0f64).sin() *
                (delta_longitude / 2.0f64);

        let c = 2.0f64 * a.sqrt().atan2((1.0f64 - a).sqrt());
        
        Kilometers(EARTH_RADIUS_IN_KM * c)
    }
}
