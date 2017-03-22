// Copyright 2017 Jose Narvaez. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error as IoError;
use std::path::Path;
use std::fmt;
use std::error;
use std::convert::From;

use serde_json::Error as JsonError;
use serde_json;

use customer::Customer;
use customer::CustomerList;
use customer_datasource::CustomerDatasource;

/// Struct abstracting the idea of a JSON file containing customer data.
///
/// It's an implementation of the `CustomerDatasource` trait allowing
/// us to retrive customer data from a file on disk with a list of
/// customers separated by new lines each one of them encoded in a JSON
/// object literal with the format:
///
/// ```json
/// {
///     "latitude": "52.833502",
///     "user_id": 25,
///     "name": "David Behan",
///     "longitude": "-8.522366"
/// }
/// ```
///
/// # Examples
///
/// ```
/// let customers_json_file = CustomerJsonFile::new(Path::new(input_file_path));
///
/// // Errors handling omitted for brevity
/// let customer_list = customers_json_file.customers().unwrap()
///
/// // or you can build a `CustomerLocator` from it (again ommiting error handling for brevity)
/// let locator = CustomerLocator::from_source(customers_json_file).unwrap();
/// ```
///
/// # Errors
/// There are primarly two kinds operations in which the JSON file customer importing
/// might fail. One is when opening the underlying file in which it will return an
/// instance of `std::io::Error`. The second case is when parding the actual JSON
/// data from the contents of the file which can come in the form of
/// `serde_json::Error`
pub struct CustomerJsonFile<'f> {
    file_path: &'f Path,
}

impl<'f> CustomerJsonFile<'f> {
    pub fn new(file_path: &'f Path) -> CustomerJsonFile<'f> {
        CustomerJsonFile { file_path: file_path }
    }
}

/// An error encapsulating the things that can go wrong when trying to open and/or
/// parse a JSON file and build a `CustomerList`.
#[derive(Debug)]
pub enum CustomerJsonFileError {
    Io(IoError),
    Json(JsonError)
}

impl fmt::Display for CustomerJsonFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomerJsonFileError::Io(ref err) => write!(f, "Customer Json file IO error: {}", err),
            CustomerJsonFileError::Json(ref err) => write!(f, "Customer Json file parsing error: {}", err)
        }
    }
}

impl error::Error for CustomerJsonFileError {
    fn description(&self) -> &str {
        match *self {
            CustomerJsonFileError::Io(ref err) => err.description(),
            CustomerJsonFileError::Json(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CustomerJsonFileError::Io(ref err) => Some(err),
            CustomerJsonFileError::Json(ref err) => Some(err)
        }
    }
}

impl From<IoError> for CustomerJsonFileError {
    fn from(err: IoError) -> Self {
        CustomerJsonFileError::Io(err)
    }
}

impl From<JsonError> for CustomerJsonFileError {
    fn from(err: JsonError) -> Self {
        CustomerJsonFileError::Json(err)
    }
}

impl<'f> CustomerDatasource for CustomerJsonFile<'f> {
    type Err = CustomerJsonFileError;

    fn customers(&self) -> Result<CustomerList, Self::Err> {
        let file = File::open(self.file_path)?;
        let reader = BufReader::new(file);
        let mut customers = Vec::new();
        for line in reader.lines() {
            let customer: Customer = serde_json::from_str(&line?)?;
            customers.push(customer);
        }

        Ok(CustomerList::from_vec(customers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use customer::Customer;
    use customer::CustomerList;
    use location::Location;

    const CUSTOMERS_OK_JSON_FILE: &str = "tests/fixtures/customers.json";
    const CUSTOMERS_BAD_JSON_FILE: &str = "tests/fixtures/customers_malformed.json";

    #[test]
    fn customer_json_file_builds_a_customer_list_from_a_json_file() {
        let customers_json_file = CustomerJsonFile::new(Path::new(CUSTOMERS_OK_JSON_FILE));

        let expected_customers = CustomerList::from_vec(vec![
            Customer::new(1, "Jose Narvaez", &Location::new(52.986375, -6.043701)),
            Customer::new(2, "Carlos Narvaez", &Location::new(51.92893, -10.27699)),
            Customer::new(3, "Maholys Narvaez", &Location::new(51.8856167, -10.4240951))
        ]);

        let actual_customers = customers_json_file.customers().unwrap();
        assert_eq!(expected_customers, actual_customers);
    }

    #[test]
    fn customer_json_file_returns_io_error_when_io_occurs() {
        let customers_json_file = CustomerJsonFile::new(Path::new("unexistent_customer_file.json"));

        match customers_json_file.customers() {
            Err(CustomerJsonFileError::Io(_)) => assert!(true),
            Err(CustomerJsonFileError::Json(_)) => assert!(false, "this was not supposed to return json error"),
            Ok(_) => assert!(false, "this was supposed to fail")
        }
    }

    #[test]
    fn customer_json_file_returns_json_error_when_malformed_json_occurs() {
        let customers_json_file = CustomerJsonFile::new(Path::new(CUSTOMERS_BAD_JSON_FILE));

        match customers_json_file.customers() {
            Err(CustomerJsonFileError::Io(_)) => assert!(false, "this was not supposed to return io error"),
            Err(CustomerJsonFileError::Json(_)) => assert!(true),
            Ok(_) => assert!(false, "this was supposed to fail")
        }
    }
}
