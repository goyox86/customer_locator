// Copyright 2017 Jose Narvaez. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error;

use customer::CustomerList;

///
/// Trait used to decouple the format of the customer's file
/// from the process of building of `CustomerLocator` instances
/// lists. Anything that implements this trait can be used as a
/// source to a `CustomerLocator`.
///
/// This is one of the core tenets of the library as it allows
/// implementors to extend the range of supported input formats
/// allowed. `CustomerDatasource` has a single `customers` method
/// that returns a `CustomerList`.
///
/// # Examples
///
/// ```
/// // Abstracts the idea of a JSON file with customer data.
/// pub struct CustomerJSONFile<'f> {
///     file_path: &'f Path,
/// }
///
/// // Some Errors might happen in the concrete implementation.
/// pub enum CustomerJsonFileError {
///     Io(IoError),
///     Json(JsonError)
/// }
///
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
    type Err: Error;
    fn customers(&self) -> Result<CustomerList, Self::Err>;
}