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

pub struct CustomerList(Vec<Customer>);

impl CustomerList {
    pub fn from_vec(customers: Vec<Customer>) -> CustomerList {
        CustomerList(customers)
    }

    pub fn sort_by_user_id(&mut self) {
         self.0.as_mut_slice().sort_by(|first, second| first.user_id.cmp(&second.user_id));
    }
}

impl IntoIterator for CustomerList {
    type Item = Customer;
    type IntoIter = ::std::vec::IntoIter<Customer>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
