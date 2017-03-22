// Copyright 2017 Jose Narvaez. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms

use customer::Customer;
use customer::CustomerList;
use location::Location;
use units::Kilometers;
use customer_datasource::{CustomerDatasource};

/// Struct used to lookup customers in different locations.
///
/// It's main responsibility is to serve as a namespace in
/// which to hold all the geographic calculations done
/// between `Customer`s and `Location`s. It does hold a
/// list `CustomerList` which uses as a buffer of customers
/// on which the calculations are gonna be perfomed. However
/// because of the existence of `CustomerList` it remains
/// decoupled from the concrete implementetion choosen for
/// the `Customer`s list.
///
/// # Examples
///
/// You can explicitly create a [`CustomerLocator`] with [`new`]:
///
/// ```
/// let dublin =  Location::dublin();
/// let santiago =  Location::new(33.4489f64, 70.6693f64);
///
/// let jose: Customer = Customer::new(1, String::from("Jose Narvaez"), dublin);
/// let carlos: Customer = Customer::new(2, String::from("Carlos Narvaez"), santiago);
///
/// let customer_list = CustomerList::from_vec(vec![jose, carlos]);
///
/// let locator = CustomerLocator::new(customer_list);
/// ```
#[derive(Debug, PartialEq)]
pub struct CustomerLocator {
    customers: CustomerList,
}

impl CustomerLocator {
    /// Constructs a new `CustomerLocator` given a raw `CustomerList`.
    ///
    /// # Examples
    ///
    /// You can explicitly create a [`CustomerLocator`] with [`new`]:
    ///
    /// ```
    /// let dublin =  Location::dublin();
    /// let santiago =  Location::new(33.4489f64, 70.6693f64);
    ///
    /// let jose: Customer = Customer::new(1, String::from("Jose Narvaez"), dublin);
    /// let carlos: Customer = Customer::new(2, String::from("Carlos Narvaez"), santiago);
    ///
    /// let customer_list = CustomerList::from_vec(vec![jose, carlos]);
    ///
    /// let locator = CustomerLocator::new(customer_list);
    /// ```
    pub fn new(customers: CustomerList) -> CustomerLocator {
        CustomerLocator { customers: customers }
    }

    /// Constructs a new `CustomerLocator` given a type that implements
    /// the `CustomerDatasource` trait.
    ///
    /// Along with `CustomerDatasource` this is the main reason we can
    /// support different formats of customer data maintaining the locator
    /// decoupled from the way we import and build `Customer` instances.
    /// This allows us to support many different formats by creating a
    /// concrete type that implements `CustomerDatasource`.
    ///
    /// # Examples
    ///
    /// ```
    /// // we are using a JSON customer file for demostration purposes.
    /// let customers_json_file = CustomerJSONFile::new(Path::new("customers.json"));
    ///
    /// // error handling skipped for brevity
    /// locator = CustomerLocator::from_source(customers_json_file).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// While buidling a `CustomerLocator` from a `CustomerDatasource` many things can
    /// go wrong as it may potentially involve doing a bunch of IO and parsing of the
    /// data in order to build `Customer` instances. `CustomerDatasource` has an `Err`
    /// associated type which has to be provided and will be built accordingly.
    ///
    pub fn from_source<S: CustomerDatasource>(source: S) -> Result<CustomerLocator, S::Err> {
        match source.customers() {
            Ok(customer_list) => Ok(Self::new(customer_list)),
            Err(err) => Err(err)
        }
    }

    ///
    /// Returns a `CustomerList` with all the customers
    /// from the internal `CustomerList` that are within
    /// the area of the `radius` in `Kilomenters` of the given
    /// `Location` in `location`.
    ///
    /// # Examples
    ///
    /// ```
    /// // we are using a JSON customer file for demostration purposes.
    /// let customers_json_file = CustomerJsonFile::new(Path::new("customers.json"));
    ///
    /// // error handling skipped for brevity
    /// locator = CustomerLocator::from_source(customers_json_file).unwrap();
    /// ```
    pub fn locate_within(&self, radius: &Kilometers, location: &Location) -> CustomerList {
        let customers_vec = self.customers
            .clone()
            .into_iter()
            .filter(|customer| customer.distance_from(location) < *radius)
            .collect::<Vec<Customer>>();

        CustomerList::from_vec(customers_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use customer::CustomerList;
    use customer_datasource::CustomerDatasource;
    use std::{error,fmt};

    // Boilerplate so we statisfy all trait bounds
    #[derive(Debug)]
    struct DummyCustomersDataFile {
        should_fail: bool
    }

    impl DummyCustomersDataFile {
        fn new(should_fail: bool) ->  DummyCustomersDataFile {
            DummyCustomersDataFile { should_fail: should_fail }
        }
    }

    impl fmt::Display for DummyCustomersDataFile {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "DummyCustomersDataFile")
        }
    }

    #[derive(Debug, PartialEq)]
    struct DummyCustomersDataFileError(String);

    impl fmt::Display for DummyCustomersDataFileError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
               DummyCustomersDataFileError(ref err) => write!(f, "DummyCustomersDataFileError error {}", err)
            }
        }
    }

    impl error::Error for DummyCustomersDataFileError {
        fn description(&self) -> &str {
            "DummyCustomersDataFileError"
        }

        fn cause(&self) -> Option<&error::Error> {
            Some(self)
        }
    }

    impl CustomerDatasource for DummyCustomersDataFile {
        type Err = DummyCustomersDataFileError;

        fn customers(&self) -> Result<CustomerList, Self::Err> {
            if self.should_fail {
                return Err(DummyCustomersDataFileError(String::from("unrecoverable error")))
            }

            Ok(generate_customer_list())
        }
    }

    // helper functions
    fn generate_customer_list() -> CustomerList {
        let santiago = Location::new(-33.4489, -70.6693);
        let jose = Customer::new(3, "Jose Narvaez", &Location::dublin());
        let carlos = Customer::new(2, "Carlos Narvaez", &santiago);
        CustomerList::from_vec(vec![carlos, jose])
    }

    // actual tests
    #[test]
    fn new_builds_a_correct_instance_from_a_customer_list() {
        let customer_list = generate_customer_list();
        let expected_locator = CustomerLocator { customers: customer_list.clone() };
        let actual_locator = CustomerLocator::new(customer_list);

        assert_eq!(expected_locator, actual_locator);
    }

    #[test]
    fn from_source_builds_a_correct_instance_from_any_type_impl_datasource() {
        let expected_locator = CustomerLocator { customers: generate_customer_list() };
        let actual_locator = CustomerLocator::from_source(DummyCustomersDataFile::new(false)).unwrap();

        assert_eq!(expected_locator, actual_locator);
    }

    #[test]
    fn from_source_propagates_the_error_from_the_datasource() {
        let expected_error = DummyCustomersDataFileError(String::from("unrecoverable error"));
        let actual_error = CustomerLocator::from_source(DummyCustomersDataFile::new(true)).unwrap_err();

        assert_eq!(expected_error, actual_error);
    }

    #[test]
    fn locate_within_locates_the_users_within_the_give_radius() {
        let all_customers = CustomerList::from_vec(vec![
            Customer::new(1, "Ian Kehoe", &Location::new(53.2451022, -6.238335)),
            Customer::new(2, "Nora Dempsey", &Location::new(53.1302756, -6.2397222)),
            Customer::new(3, "Theresa Enright", &Location::new(53.1229599, -6.2705202)),
            Customer::new(4, "Eoin Ahearn" , &Location::new(54.0894797, -6.18671)),
            Customer::new(5, "Richard Finnegan" , &Location::new(53.008769, -6.1056711)),
            Customer::new(6, "Christina McArdle", &Location::new(52.986375, -6.043701)),
            Customer::new(7, "Olive Ahearn", &Location::new(53.00, -7.00)),
            Customer::new(8, "Michael Ahearn", &Location::new(52.966, -6.463)),
            Customer::new(9, "Patricia Cahill", &Location::new(54.180238, -5.920898)),
            Customer::new(10, "Eoin Gallagher",&Location::new(54.080556, -6.361944)),
            Customer::new(11, "Rose Enright", &Location::new(54.133333, -6.433333)),
            Customer::new(12, "Stephen McArdle", &Location::new(53.038056, -7.653889)),
            Customer::new(13, "Oliver Ahearn", &Location::new(53.74452, -7.11167)),
            Customer::new(14, "Nick Enright", &Location::new(53.761389, -7.2875)),
            Customer::new(15, "Alan Behan", &Location::new(53.1489345, -6.8422408)),
            Customer::new(16, "Lisa Ahearn", &Location::new(53.0033946, -6.3877505))
        ]);

        let expected_customers = CustomerList::from_vec(vec![
            Customer::new(1, "Ian Kehoe", &Location::new(53.2451022, -6.238335)),
            Customer::new(2, "Nora Dempsey", &Location::new(53.1302756, -6.2397222)),
            Customer::new(3, "Theresa Enright", &Location::new(53.1229599, -6.2705202)),
            Customer::new(5, "Richard Finnegan" , &Location::new(53.008769, -6.1056711)),
            Customer::new(6, "Christina McArdle", &Location::new(52.986375, -6.043701)),
            Customer::new(8, "Michael Ahearn", &Location::new(52.966, -6.463)),
            Customer::new(15, "Alan Behan", &Location::new(53.1489345, -6.8422408)),
            Customer::new(16, "Lisa Ahearn", &Location::new(53.0033946, -6.3877505))
        ]);

        let locator = CustomerLocator::new(all_customers);
        let actual_customers = locator.locate_within(&Kilometers(50.00), &Location::dublin());
        assert_eq!(expected_customers, actual_customers);
    }
}