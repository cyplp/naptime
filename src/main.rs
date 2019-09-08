#[macro_use]
extern crate clap;
extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::{thread, time};

use clap::{App, Arg};

pub mod napstruct;

pub fn parse(filename: &str, params: &HashMap<&str, &str>) -> Option<Vec<napstruct::Request>> {
    let mut result: Vec<napstruct::Request> = Vec::new();
    let file = File::open(filename);
    let mut tmp = Vec::<String>::new();

    for line in BufReader::new(file.unwrap()).lines() {
        let current = line.unwrap();
        if current.starts_with('#') {
            if !tmp.is_empty() {
                let mut request = napstruct::Request::from_vec(tmp);
                request.fix_params(&params);
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
                .help("File containing requests with restclient.el format")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("interval")
                .short("i")
                .long("interval")
                .help("interval between two requests in milliseconds")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("select")
                .short("s")
                .long("select")
                .help("select only some requests as <index1,index2,index3...> start at 1")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("parameter")
                .short("p")
                .long("parameter")
                .help("")
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();

    let mut no = napstruct::napoption::NapOptions::new();

    let str_interval: u64 = matches.value_of("interval").unwrap_or("0").parse().unwrap();
    let interval = time::Duration::from_millis(str_interval);
    no.set_interval(interval);

    // TODO improve select parsing
    let str_select = matches.value_of("select").unwrap_or("");
    let selected: Vec<usize> = str_select
        .split(",")
        .map(|current| current.parse().unwrap())
        .collect();

    for select in selected{
        no.add_selected(select);
    }

    // TODO function
    let mut params: HashMap<&str, &str> = HashMap::new();
    for current in matches.values_of("parameter").unwrap() {
        let tmp = current.split("=").collect::<Vec<&str>>();
        params.insert(tmp[0], tmp[1]);
    }

    let filename = matches.value_of("file").unwrap();
    let requests = parse(filename, &params).unwrap();

    let mut first = true;
    for (num, request) in requests.iter().enumerate() {
        if matches.is_present("select") {
            if !no.selecteds.contains(&(num + 1)) {
                continue;
            }
        }

        let mut res = request.send();

        if !first {
            println!("");
        }
        first = false;

        println!("{}", res.text().unwrap());
        println!("// {} {}", request.verb, request.url);
        println!("// {:?} {}", res.version(), res.status());

        for (key, value) in res.headers().iter() {
            println!("// {}: {}", key, value.to_str().unwrap());
        }

        if matches.is_present("interval") {
            thread::sleep(no.interval);
        }
    }
}
