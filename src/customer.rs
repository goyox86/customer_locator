use location::Location;
use units::Kilometers;

#[derive(Clone, Serialize, Deserialize)]
pub struct Customer {
    pub user_id: i64,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Customer {
    #![allow(dead_code)]
    pub fn new(user_id: i64, name: String, location: Location) -> Customer {
        Customer {
            user_id: user_id,
            name: name,
            latitude: location.latitude,
            longitude: location.longitude,
        }
    }

    pub fn location(&self) -> Location {
        Location::new(self.latitude, self.longitude)
    }

    pub fn distance_from(&self, location: &Location) -> Kilometers {
        self.location().distance_from(&location)
    }
}
