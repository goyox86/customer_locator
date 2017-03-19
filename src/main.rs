#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate clap;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

mod customer;
mod location;

use customer::Customer;
use location::Location;
use location::Kilometers;

trait CustomerSource {
    fn customers(&self) -> Vec<Customer>;
}

struct CustomerJSONFile<'f> {
    file_path: &'f Path
}

impl<'f> CustomerJSONFile<'f> {
    pub fn new(file_path: &'f Path) -> CustomerJSONFile<'f> {
        CustomerJSONFile { file_path: file_path }
    }
}

impl<'f> CustomerSource for CustomerJSONFile<'f> {
    fn customers(&self) -> Vec<Customer> {
        let file = File::open(self.file_path).unwrap();
        let reader = BufReader::new(file);
        let mut customers = Vec::new();
        for line in reader.lines() {
            let customer: Customer = serde_json::from_str(&line.unwrap()).unwrap();
            customers.push(customer);
        }

        customers
    }
}

struct CustomerLocator {
    pub customers: Vec<Customer>
}

impl CustomerLocator {
    fn new(customers: Vec<Customer>) -> CustomerLocator {
        CustomerLocator { customers: customers }
    }

    pub fn from_source<S: CustomerSource>(source: S) -> CustomerLocator {
        Self::new(source.customers())
    }

    pub fn locate_within(&self, radius: &Kilometers, location: &Location) -> Vec<Customer> {
        self.customers.clone().into_iter()
            .filter(|customer| customer.distance_from(location) < *radius)
            .collect::<Vec<Customer>>()
    }
}

fn main() {
    let customers_json_file = CustomerJSONFile::new(Path::new("data/customers.json"));
    let locator = CustomerLocator::from_source(customers_json_file);

    let dublin = Location::new(53.3393f64, -6.2576841f64);
    let mut customers_within_100_km = locator.locate_within(&Kilometers(100f64), &dublin);

    customers_within_100_km.as_mut_slice().
        sort_by(|first, second| first.user_id.cmp(&second.user_id));

    for customer in customers_within_100_km {
        let dist_from_dublin = customer.location().distance_from(&dublin);
        println!("Distance from Dublin for {}: {} is {}", customer.name, customer.user_id, dist_from_dublin);
    }
}


