use customer::Customer;
use location::Location;
use units::Kilometers;
use customer_datasource::CustomerDatasource;

pub struct CustomerLocator {
    pub customers: Vec<Customer>,
}

impl CustomerLocator {
    fn new(customers: Vec<Customer>) -> CustomerLocator {
        CustomerLocator { customers: customers }
    }

    pub fn from_source<S: CustomerDatasource>(source: S) -> CustomerLocator {
        Self::new(source.customers())
    }

    pub fn locate_within(&self, radius: &Kilometers, location: &Location) -> Vec<Customer> {
        self.customers
            .clone()
            .into_iter()
            .filter(|customer| customer.distance_from(location) < *radius)
            .collect::<Vec<Customer>>()
    }
}
