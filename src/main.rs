extern crate clap;
extern crate http;
extern crate hyper;
extern crate futures;
extern crate hyper_tls;
extern crate tokio;


use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

use clap::{App, Arg};
use hyper::Client;
use hyper::Request as hr;

use futures::{future, Future};
use hyper_tls::HttpsConnector;

const APP: &str = "naptime";
const VERSION: &str = "1.0";

#[derive(Debug)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn new(name: String, value: String) -> Header {
        Header { name, value }
    }
}

#[derive(Debug)]
pub struct Request {
    verb: String,
    url: String,
    headers: Vec<Header>,
    body: String,
}

impl Request {
    pub fn new(verb: String, url: String) -> Request {
        Request {
            verb,
            url,
            headers: Vec::new(),
            body: String::new(),
        }
    }

    pub fn add_header(&mut self, header: Header) {
        self.headers.push(header);
    }

    pub fn add_body(&mut self, body: String) {
        self.body = body;
    }

    pub fn is_empty(&self) -> bool {
        self.verb.is_empty() || self.url.is_empty()
    }
    pub fn run(&self) {
        let mut req = hr::builder();
        req.method(self.verb.as_str());
        req.uri(self.url.as_str());
        for header in &self.headers {
            req.header(header.name.as_str(), header.value.as_str());
        }
        let todo = req.body(hyper::Body::from(self.body.clone())).unwrap();
        println!("{:?}", self);
        println!("{:?}", todo);
        tokio::run(future::lazy(|| {
            // 4 is number of blocking DNS threads
            let https = HttpsConnector::new(4).unwrap();
            let client = Client::builder().build::<_, hyper::Body>(https);
            client.request(todo)
                .map(|res| println!("{:?}", res.body()))
                .map_err(|e| println!("request error: {}", e))
        }));
    }
}

pub fn is_header(line: &str) -> bool {
    line.contains(": ")
}

pub fn vec2request(buffer: Vec<String>) -> Request {
    let first = buffer[0].split(' ').collect::<Vec<&str>>();

    let mut request = Request::new(first[0].to_string(), first[1..].join(" "));

    let mut body = false;
    let mut tmp: Vec<String> = Vec::new();

    for line in buffer.iter().skip(1) {
        if !body {
            if is_header(&line) {
                let headers = line.split(": ").collect::<Vec<&str>>();
                request.add_header(Header::new(headers[0].to_string(), headers[1..].join(": ")));
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

pub fn parse(filename: &str) -> Option<Vec<Request>> {
    let mut result: Vec<Request> = Vec::new();
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
