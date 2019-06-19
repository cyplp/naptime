extern crate clap;
extern crate http;
extern crate hyper;
extern crate futures;
extern crate hyper_tls;
extern crate tokio;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

//use hyper::Request as hr;

use clap::{App, Arg};

pub mod napstruct;

//use self::napstruct::Request;
//use self::napstruct::Header;

const APP: &str = "naptime";
const VERSION: &str = "1.0";


pub fn is_header(line: &str) -> bool {
    line.contains(": ")
}

pub fn vec2request(buffer: Vec<String>) -> napstruct::Request {
    let first = buffer[0].split(' ').collect::<Vec<&str>>();

    let mut request = napstruct::Request::new(first[0].to_string(), first[1..].join(" "));

    let mut body = false;
    let mut tmp: Vec<String> = Vec::new();

    for line in buffer.iter().skip(1) {
        if !body {
            if is_header(&line) {
                let headers = line.split(": ").collect::<Vec<&str>>();
                request.add_header(napstruct::napheader::Header::new(headers[0].to_string(), headers[1..].join(": ")));
            } else {
                body = true;
                tmp.push(line.to_string());
            }
        } else {
            tmp.push(line.to_string());
        }
    }
    if !tmp.is_empty() {
        request.add_body(tmp.join("\n"));
    }
    request
}

pub fn parse(filename: &str) -> Option<Vec<napstruct::Request>> {
    let mut result: Vec<napstruct::Request> = Vec::new();
    let file = File::open(filename);
    let mut tmp = Vec::<String>::new();

    for line in BufReader::new(file.unwrap()).lines() {
        let current = line.unwrap();
        if current.starts_with('#') {
            if !tmp.is_empty() {
                let request = vec2request(tmp);
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
    println!("{:?}", requests);
    for request in requests {
        request.run();
    }
}
