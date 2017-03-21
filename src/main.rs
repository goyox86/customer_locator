#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate clap;

use std::path::Path;
use std::str::FromStr;

use clap::{App, Arg};

mod customer;
mod customer_locator;
mod customer_datasource;
mod location;
mod units;

use location::Location;
use units::Kilometers;
use customer_locator::CustomerLocator;
use customer_datasource::CustomerJSONFile;

const DEFAULT_ARG_CUSTOMERS_FILE: &str = "data/customers.json";
const DEFAULT_ARG_RADIUS_IN_KM: &str = "100";
const DEFAULT_ARG_LOCATION: &str = "53.3393,-6.2576841"; // Dublin

fn main() {
    let matches = App::new("CustomerLocator")
        .version("0.1.0")
        .author("Jose Narvaez. <goyox86@gmail.com>")
        .about("Locates customers near a given location.")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("The input file with the customers")
            .default_value(DEFAULT_ARG_CUSTOMERS_FILE)
            .takes_value(true))
        .arg(Arg::with_name("radius")
            .short("r")
            .long("radius")
            .value_name("RADIUS")
            .help("The radius of the search in Kilometers")
            .default_value(DEFAULT_ARG_RADIUS_IN_KM)
            .takes_value(true))
        .arg(Arg::with_name("location")
            .short("l")
            .long("location")
            .value_name("LOCATION")
            .help("The location for what customers are gonna be located. In the format latitude,longitude.")
            .default_value(DEFAULT_ARG_LOCATION)
            .takes_value(true))
        .get_matches();

    // Parsing input file name
    let input_file_path = matches.value_of("file").unwrap();

    // Parsing radius of the search
    let radius_str = matches.value_of("radius").unwrap();
    let radius = match f64::from_str(radius_str) {
        Ok(radius) => radius,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    // Parsing the location
    let location_str = matches.value_of("location").unwrap();
    let location = match Location::from_str(location_str) {
        Ok(location) => location,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let customers_json_file = CustomerJSONFile::new(Path::new(input_file_path));
    let locator = CustomerLocator::from_source(customers_json_file);

    let mut customers = locator.locate_within(&Kilometers(radius), &location);
    customers.as_mut_slice().sort_by(|first, second| first.user_id.cmp(&second.user_id));

    for customer in customers {
        let dist_from_dublin = customer.location().distance_from(&location);
        println!("Distance from Dublin for {}: {} is {}",
                 customer.name,
                 customer.user_id,
                 dist_from_dublin);
    }
}