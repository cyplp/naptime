#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::{thread, time};

use clap::{App, Arg};

pub mod napstruct;

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
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
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
                .help("interval between two requests in milliseconds")
                .takes_value(true))
        .arg(
            Arg::with_name("select")
                .short("s")
                .long("select")
                .help("select only some requests as <index1,index2,index3...> start at 1")
                .takes_value(true))
        .get_matches();

    let filename = matches.value_of("file").unwrap();

    let str_interval: u64 = matches.value_of("interval").unwrap_or("0").parse().unwrap();
    let interval = time::Duration::from_millis(str_interval);

    let str_select = matches.value_of("select").unwrap_or("");
    let selected: Vec<usize> = str_select.split(",").map(|current| current.parse().unwrap()).collect();

    let requests = parse(filename).unwrap();

    for (num, request) in requests.iter().enumerate() {
        if matches.is_present("select") {
            if !selected.contains(&(num + 1)) {
                continue;
            }
        }
        request.run();
        if matches.is_present("interval"){
            thread::sleep(interval);
        }
    }
}
