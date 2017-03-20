use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use serde_json;

use customer::Customer;

pub trait CustomerDatasource {
    fn customers(&self) -> Vec<Customer>;
}

pub struct CustomerJSONFile<'f> {
    file_path: &'f Path,
}

impl<'f> CustomerJSONFile<'f> {
    pub fn new(file_path: &'f Path) -> CustomerJSONFile<'f> {
        CustomerJSONFile { file_path: file_path }
    }
}

impl<'f> CustomerDatasource for CustomerJSONFile<'f> {
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
