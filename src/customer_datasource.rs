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
use std::path::Path;
use std::io::Error as IoError;
use std::fmt;
use std::error;
use std::convert::From;

use serde_json;
use serde_json::Error as JsonError;

use customer::Customer;
use customer::CustomerList;

///
/// Trait used to decouple the format of the customer's list
/// from the building of `CustomerList` lists.
///
/// This is one of the core tenets of the library as it allows
/// implementors to extend the range of supported input formats
/// allowed. `CustomerDatasource` has a single `customers` method
/// that returns a `CustomerList`.App
///
/// # Examples
///
/// You can explicitly create a [`CustomerLocato`] with [`new`]:
///
/// ```
/// // Abstracts the idea of a JSON file with customer data.
/// pub struct CustomerJSONFile<'f> {
///     file_path: &'f Path,
/// }
///
/// // Some Errors might happen in the concrete implementation so
/// pub enum CustomerJsonFileError {
///     Io(IoError),
///     Json(JsonError)
/// }

/// // implementing `CustomerDatasource`
/// impl<'f> CustomerDatasource for CustomerJSONFile<'f> {
///     type Err = CustomerJsonFileError;
///
///     fn customers(&self) -> Result<CustomerList, Self::Err> {
///         let file = File::open(self.file_path)?;
///         let reader = BufReader::new(file);
///         let mut customers = Vec::new();
///         for line in reader.lines() {
///             let customer: Customer = serde_json::from_str(&line?)?;
///             customers.push(customer);
///         }
///
///         Ok(CustomerList::from_vec(customers))
///     }
/// }
/// 
/// // Now you can get the customers from the `CustomerJSONFile`
/// let customers_json_file = CustomerJSONFile::new(Path::new(input_file_path));
///
/// // Errors handling ommited for brevity
/// let customer_list = customers_json_file.customers().unwrap();
/// ```
pub trait CustomerDatasource {
    type Err: error::Error;
    fn customers(&self) -> Result<CustomerList, Self::Err>;
}

/// Struct abstracting the idea of a JSON file containg customer data.
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
/// let customers_json_file = CustomerJSONFile::new(Path::new(input_file_path));
///
/// // Errors handling ommited for brevity
/// let customer_list = customers_json_file.customers().unwrap()
///
/// // or you can build a `CustomerLocator` from it (again ommiting error handling for brevity)
/// let locator = CustomerLocator::from_source(customers_json_file).unwrap();
/// ```
///
/// # Errors
/// There are primarly two kinds operations in which the JSON file customer importing
/// might fail. One is when opening the underlying file in which it will return an instance
/// of `std::io::Error`. The second case is when parding the actual JSON data from the
/// contents of the file which can come in the form of serde_json::Error as JsonError;`
///
pub struct CustomerJSONFile<'f> {
    file_path: &'f Path,
}

impl<'f> CustomerJSONFile<'f> {
    pub fn new(file_path: &'f Path) -> CustomerJSONFile<'f> {
        CustomerJSONFile { file_path: file_path }
    }
}

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

impl<'f> CustomerDatasource for CustomerJSONFile<'f> {
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
