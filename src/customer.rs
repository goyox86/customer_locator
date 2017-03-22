// Copyright 2017 Jose Narvaez. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use location::Location;
use units::Kilometers;

///
/// Struct representing a single entry of customer data.
///
/// It is mainly used to load the customers data coming from
/// different datasources. It by default derives serde's JSON
/// Serialize and Deserialize traits so it can be easily built
/// from JSON and written to JSON strings.
///
/// # Examples
///
/// You can explicitly create a [`Customer`] with [`new`]:
///
/// ```
/// let user_id = 1i64;
/// let name = String::from("Jose Narvaez")
/// let location: Location::new(53.3393, -6.2576841);
///
/// let customer: Customer = Customer::new(user_id, name, location);
/// ```
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub user_id: i64,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Customer {
    /// Constructs a new `Customer` given the `user_id`, `name` and `location`.
    ///
    /// # Examples
    ///
    /// ```
    /// let user_id = 1i64;
    /// let name = String::from("Jose Narvaez")
    /// let location: Location::new(53.3393, -6.2576841);
    ///
    /// let customer: Customer = Customer::new(user_id, name, location);
    /// ```
    #[allow(dead_code)]
    pub fn new(user_id: i64, name: &str, location: &Location) -> Customer {
        Customer {
            user_id: user_id,
            name: name.into(),
            latitude: location.latitude,
            longitude: location.longitude,
        }
    }

    /// Returns a `Location` object built from customer's latitude and longitude.
    ///
    /// # Examples
    ///
    /// ```
    /// let user_id = 1i64;
    /// let name = String::from("Jose Narvaez")
    /// let location: Location::new(53.3393, -6.2576841);
    ///
    /// let customer: Customer = Customer::new(user_id, name, location);
    /// assert_eq!(customer.location(), location);
    /// ```
    pub fn location(&self) -> Location {
        Location::new(self.latitude, self.longitude)
    }

    /// Returns a the distance in `Kilometers` between the customer's `Location`
    /// and the provided one.
    ///
    /// # Examples
    ///
    /// ```
    /// let user_id = 1i64;
    /// let name = String::from("Jose Narvaez")
    /// let customer_location: Location::new(53.3393, -6.2576841);
    ///
    /// let customer: Customer = Customer::new(user_id, name, customer_location);
    /// let other_location = Location::new(-6.238335, 53.2451022);
    /// assert_eq!(customer.distance_from(other_location), Kilometers(10.553));
    /// ```
    pub fn distance_from(&self, location: &Location) -> Kilometers {
        self.location().distance_from(&location)
    }
}

/// Struct representing a list of customers.
///
/// `CustomerList` is a NewType that represents the lingua franca
/// of the library as all the operations that consume or return
/// the concept of a list of customers use it. It's main purpose
/// is to allow the internal representation of the list of customers
/// which is currently a `Vec<Customers>` to be swapped without
/// causing a cascade of changes across all the crate.
/// 
/// It importantly holds operations that are related to a set of
/// customers that are not related to importing and locating them
/// as many other operations and/or operators may be devised on
/// the future whose have not been devised by the implementor.
///
/// # Examples
///
/// You can explicitly create a [`CustomerList`] with `from_vec` like:
///
/// ```
/// let dublin =  Location::dublin();
/// let santiago =  Location::new(33.4489f64, 70.6693f64);
///
/// let jose: Customer = Customer::new(1i64, String::from("Jose Narvaez"), dublin);
/// let carlos: Customer = Customer::new(2i64, String::from("Carlos Narvaez"), santiago);
///
/// let customer_list = CustomerList::from_vec(vec![jose, carlos]);
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub struct CustomerList(Vec<Customer>);

impl CustomerList {
    /// Constructs a `CustomerList` instance from a vector of customers.
    ///
    ///  # Examples
    ///
    /// ```
    /// let dublin =  Location::dublin();
    /// let santiago =  Location::new(33.4489f64, 70.6693f64);
    ///
    /// let jose: Customer = Customer::new(1i64, String::from("Jose Narvaez"), dublin);
    /// let carlos: Customer = Customer::new(2i64, String::from("Carlos Narvaez"), santiago);
    ///
    /// let customer_list = CustomerList::from_vec(vec![jose, carlos]);
    /// ```
    pub fn from_vec(customers: Vec<Customer>) -> CustomerList {
        CustomerList(customers)
    }

    /// Sorts in-place the `CustomerList` by the `user_id`.
    ///
    ///  # Examples
    ///
    /// ```
    /// let dublin =  Location::dublin();
    /// let santiago =  Location::new(-33.4489f64, -70.6693f64);
    ///
    /// let jose: Customer = Customer::new(2i64, String::from("Jose Narvaez"), dublin);
    /// let carlos: Customer = Customer::new(1i64, String::from("Carlos Narvaez"), santiago);
    ///
    /// let customer_list = CustomerList::from_vec(vec![jose.clone(), carlos.clone()]);
    ///
    /// customers.sort_by_user_id();
    /// assert_eq!(customer_list, CustomerList::from_vec(vec![carlos, jose]));
    /// ```
    pub fn sort_by_user_id(&mut self) {
         self.0.as_mut_slice().sort_by(|first, second| first.user_id.cmp(&second.user_id));
    }
}

// this is to allow CustomerList instances in for loops.
impl IntoIterator for CustomerList {
    type Item = Customer;
    type IntoIter = ::std::vec::IntoIter<Customer>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Display for Customer {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "Customer(\"{}\": {}) located at ({}, {})", self.name, self.user_id, self.latitude, self.longitude)
     }
}

#[cfg(test)]
mod tests {
    use super::*;
    const NY_LAT: f64 = 40.7128f64;
    const NY_LONG: f64 = -74.0059f64;
    const DUB_NY_DIST_IN_KM: Kilometers = Kilometers(5116.751541958293);

    #[test]
    fn customer_new_builds_a_correct_instance() {
        let actual_customer = Customer::new(1000i64, "Jose Narvaez",&Location::dublin());
        let expected_customer = Customer {
            user_id: 1000i64,
            name: String::from("Jose Narvaez"),
            latitude: Location::dublin().latitude,
            longitude: Location::dublin().longitude,
        };
        assert_eq!(expected_customer, actual_customer);
    }

    #[test]
    fn customer_location_returns_a_correct_instance_of_location() {
        let expected_location = Location::dublin();
        let customer = Customer::new(1000i64, "Jose Narvaez", &expected_location);
        assert_eq!(customer.location(), expected_location);
    }

     #[test]
    fn customer_distance_from_calculates_distance_between_diff_points_long() {
        let customer = Customer::new(1000i64, "Jose Narvaez", &Location::dublin());
        let new_york = Location::new(NY_LAT, NY_LONG);
        assert_eq!(customer.distance_from(&new_york), DUB_NY_DIST_IN_KM);
    }

    #[test]
    fn customer_distance_from_calculates_distance_same_point() {
        let customer = Customer::new(1000i64, "Jose Narvaez", &Location::dublin());
        let dublin = Location::dublin();
        assert_eq!(customer.distance_from(&dublin), Kilometers(0f64));
    }

    #[test]
    fn customer_list_from_vec_builds_a_correct_instance() {
        let jose = Customer::new(1, "Jose Narvaez", &Location::dublin());
        let carlos = Customer::new(2, "Carlos Narvaez", &Location::new(-33.4489, -70.6693));
        let expected_cust_list = CustomerList(vec![jose.clone(), carlos.clone()]);
        let actual_cust_list = CustomerList::from_vec(vec![jose, carlos]);
        assert_eq!(expected_cust_list, actual_cust_list);
    }

    #[test]
    fn customer_list_sort_by_user_id_sorts_the_list_in_place() {
        let santiago = Location::new(-33.4489, -70.6693);
        let jose = Customer::new(3, "Jose Narvaez", &Location::dublin());
        let carlos = Customer::new(2, "Carlos Narvaez", &santiago);
        let maho = Customer::new(1, "Maholys Narvaez", &santiago);
        let expected_list = CustomerList(vec![maho.clone(), carlos.clone(), jose.clone()]);
        let mut sorted_list = CustomerList(vec![jose, carlos, maho]);
        sorted_list.sort_by_user_id();

        assert_eq!(expected_list, sorted_list);
    }
}