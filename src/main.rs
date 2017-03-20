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

const DEFAULT_CUSTOMERS_FILE: &str = "data/customers.json";
const DEFAULT_RADIUS: &str = "100";

fn main() {
    let matches = App::new("CustomerLocator")
                        .version("1.0")
                        .author("Jose Narvaez. <goyox86@gmail.com>")
                        .about("Locates customers near Dublin.")
                        .arg(Arg::with_name("file")
                            .short("f")
                            .long("file")
                            .value_name("FILE")
                            .help("The input file with the customers")
                            .default_value(DEFAULT_CUSTOMERS_FILE)
                            .takes_value(true))
                        .arg(Arg::with_name("radius")
                            .short("r")
                            .long("radius")
                            .value_name("RADIUS")
                            .help("The radius of the search in Kilometers")
                            .default_value(DEFAULT_RADIUS)
                            .takes_value(true))
                        .arg(Arg::with_name("debug")
                            .short("d")
                            .multiple(true)
                            .help("Turn debugging information on"))
                        .get_matches();

    // Parsing input file name
    let input_file_path = matches.value_of("file").unwrap();

    // Parsing radius of the search
    let radius_str = matches.value_of("radius").unwrap();
    let radius = match f64::from_str(radius_str) {
        Ok(radius) => radius,
        Err(err) => {
            println!("Invalid radius: {}", err);
            return;
        }
    };

    let customers_json_file = CustomerJSONFile::new(Path::new(input_file_path));
    let locator = CustomerLocator::from_source(customers_json_file);
    let dublin = Location::new(53.3393f64, -6.2576841f64);

    let mut customers = locator.locate_within(&Kilometers(radius), &dublin);
    customers.as_mut_slice().sort_by(|first, second| first.user_id.cmp(&second.user_id));

    for customer in customers {
        let dist_from_dublin = customer.location().distance_from(&dublin);
        println!("Distance from Dublin for {}: {} is {}",
                 customer.name,
                 customer.user_id,
                 dist_from_dublin);
    }
}
