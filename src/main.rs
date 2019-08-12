extern crate clap;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::{thread, time};

use clap::{App, Arg};

pub mod napstruct;

const APP: &str = "naptime";
const VERSION: &str = "1.0";

pub fn parse(filename: &str) -> Option<Vec<napstruct::Request>> {
    let mut result: Vec<napstruct::Request> = Vec::new();
    let file = File::open(filename);
    let mut tmp = Vec::<String>::new();

    for line in BufReader::new(file.unwrap()).lines() {
        let current = line.unwrap();
        if current.starts_with('#') {
            if !tmp.is_empty() {
                let request = napstruct::Request::from_vec(tmp);
                if !request.is_empty() {
                    result.push(request);
                }
                tmp = Vec::<String>::new();
            }
        } else {
            tmp.push(current);
        }
    }
    Some(result)
}

fn main() {
    let matches = App::new(APP)
        .version(VERSION)
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("File containing urls")
                .takes_value(true))
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .help("interval between two requests in seconds")
                .takes_value(true))
        .get_matches();

    let filename = matches.value_of("file").unwrap();

    let str_interval: u64 = matches.value_of("interval").unwrap_or("0").parse().unwrap();
    let interval = time::Duration::from_millis(str_interval);

    let requests = parse(filename).unwrap();

    for request in requests {
        request.run();
        thread::sleep(interval);
    }
}
