// Copyright 2017 Jose Narvaez. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed
// except according to those terms

///
/// A small CLI application to locate customers nearby a given Location.
/// Originally written to search for people in a radius of 100km of the
/// center of Dublin, Ireland.
///
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
mod customer_json_file;
mod location;
mod units;

use location::Location;
use units::Kilometers;
use customer_locator::CustomerLocator;
use customer_json_file::CustomerJsonFile;

const DEFAULT_ARG_CUSTOMERS_FILE: &'static str = "data/customers.json";
const DEFAULT_ARG_RADIUS_IN_KM: &'static str = "100";
const DEFAULT_ARG_LOCATION: &'static str = "53.3393,-6.2576841"; // Dublin

fn main() {
    let matches = App::new("CustomerLocator")
        .version("0.1.0")
        .author("Jose Narvaez. <goyox86@gmail.com>")
        .about("Locates customers near a given location. Dublin, Ireland by default")
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
        .arg(Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .help("Don't print anything to stdout. Used when benchmarking, so we don't wait on stdout flushing."))
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

    // Building our datasource (A JSON file in this case)
    let customers_json_file = CustomerJsonFile::new(Path::new(input_file_path));

    // Building the locator
    let locator = match CustomerLocator::from_source(customers_json_file) {
        Ok(locator) => locator,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let mut customers = locator.locate_within(&Kilometers(radius), &location);
    customers.sort_by_user_id();

    // this is just to be able to measure raw perf of customer parsing and actual
    // calculations excluding IO at the end.
    if matches.is_present("quiet") { return; }

    if location.is_dublin() {
        println!("Location is (Dublin, Ireland) {}.", location);
    } else {
        println!("Location is {}.", location);
    }

    for customer in customers {
        let dist_from_location = customer.distance_from(&location);
        println!("{} is {} from provided location.", customer, dist_from_location);
    }
}