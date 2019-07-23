extern crate clap;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

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
                .takes_value(true),
        )
        .get_matches();

    let filename = matches.value_of("file").unwrap();
    let requests = parse(filename).unwrap();
    for request in requests {
        request.run();
    }
}
