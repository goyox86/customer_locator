use customer::Customer;
use customer::CustomerList;
use location::Location;
use units::Kilometers;
use customer_datasource::{CustomerDatasource};

use std::error;
use std::fmt;

/// Struct used to lookup customers in different locations.
///
/// It's main responsibility is to serve as a namespace in
/// which to hold all the geographic calculations done
/// between `Customer`s and `Location`s. It does hold a
/// list `CustomerList` which uses as a buffer of customers
/// on which the calculations are gonna be perfomed. However
/// because of the existence of `CustomerList` it remains
/// decoupled from the concrete implemantetion choosen for
/// the `Customer` list.
///
/// # Examples
///
/// You can explicitly create a [`CustomerLocato`] with [`new`]:
///
/// ```
/// let dublin =  Location::dublin();
/// let santiago =  Location::new(33.4489f64, 70.6693f64);
///
/// let jose: Customer = Customer::new(1i64, String::from("Jose Narvaez"), dublin);
/// let carlos: Customer = Customer::new(2i64, String::from("Carlos Narvaez"), santiago);
///
/// let customer_list = CustomerList::from_vec(vec![jose, carlos]);
///
/// let locator = CustomerLocator::new(customer_list);
/// ```
pub struct CustomerLocator {
    pub customers: CustomerList,
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
    /// let jose: Customer = Customer::new(1i64, String::from("Jose Narvaez"), dublin);
    /// let carlos: Customer = Customer::new(2i64, String::from("Carlos Narvaez"), santiago);
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
    /// This allows us to support many different formats by icreating a
    /// concrete type that implements `CustomerDatasource`
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
    /// data in order to build `Customer` instances. There is a very generic 
    /// `CustomerLocatorError(String)` that holds a string representation of the underlying
    /// error and it is not possible to enumerate all of the possible errors that might pop
    /// up from the internal layers of an importer such as `CustomerJSONFile`.
    pub fn from_source<S: CustomerDatasource>(source: S) -> Result<CustomerLocator, CustomerLocatorError> {
        match source.customers() {
            Ok(customer_list) => Ok(Self::new(customer_list)),
            Err(err) => Err(CustomerLocatorError(format!("{}", err)))
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
    /// You can explicitly create a [`CustomerLocator`] with [`new`]:
    ///
    /// ```
    /// // we are using a JSON customer file for demostration purposes.
    /// let customers_json_file = CustomerJSONFile::new(Path::new("customers.json"));
    ///
    /// // error handling skipped for brevity
    /// locator = ustomerLocator::from_source(customers_json_file).unwrap();
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

impl fmt::Display for CustomerLocatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Customer locator error: {}", self.0)
    }
}

///
/// An error when trying to build a `CustomerLocator` instance from a `CustomerDatasource`.
///
/// It does wrap a string representation of the underlying error generated by the
/// `CustomerDatasource` implementor.
#[derive(Debug)]
pub struct CustomerLocatorError(String);

impl error::Error for CustomerLocatorError {
    fn description(&self) -> &str {
        &self.0
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(self)
    }
}