// Copyright 2017 Jose Narvaez. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

/// Struct representing a single customer data
///
/// It is mainly used to load the customers data coming from
/// different datasources. It by default derives serde's JSON
/// Serialize and Deserialize traits so it can be easily built
/// from JSON strings and saved to JSON strings
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
use std::fmt;

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
    pub fn new(user_id: i64, name: String, location: Location) -> Customer {
        Customer {
            user_id: user_id,
            name: name,
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
// is to allow the internal representation of the list of customers
/// which is currently a `Vec<Customers>` to be swapped without
/// causing a cascade of changes along all the crate.
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
#[derive(Clone)]
pub struct CustomerList(Vec<Customer>);

impl CustomerList {
    ///
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

    ///
    /// Sorts in-place the `CustomerList` by the `user_id`.
    ///
    ///  # Examples
    ///
    /// ```
    /// let dublin =  Location::dublin();
    /// let santiago =  Location::new(33.4489f64, 70.6693f64);
    ///
    /// let jose: Customer = Customer::new(2i64, String::from("Jose Narvaez"), dublin);
    /// let carlos: Customer = Customer::new(1i64, String::from("Carlos Narvaez"), santiago);
    ///
    /// let customer_list = CustomerList::from_vec(vec![jose, carlos]);
    ///
    /// customers.sort_by_user_id();
    /// assert_eq!(customer_list, CustomerList::from_vec(vec![carlos, jose]));
    /// ```
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

impl fmt::Display for Customer {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "Customer(\"{}\":{}) located at ({}, {})", self.name, self.user_id, self.latitude, self.longitude)
     }
}