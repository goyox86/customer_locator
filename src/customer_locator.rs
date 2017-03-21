use customer::Customer;
use customer::CustomerList;
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

    pub fn locate_within(&self, radius: &Kilometers, location: &Location) -> CustomerList {
        let customers_vec = self.customers
            .clone()
            .into_iter()
            .filter(|customer| customer.distance_from(location) < *radius)
            .collect::<Vec<Customer>>();

        CustomerList::from_vec(customers_vec)
    }
}
