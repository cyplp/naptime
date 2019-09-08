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

// TODO move this in another file
pub trait ResponseExt {
    fn display_body(&mut self);
    fn display_headers(&self);
}

// TODO move this in another file
impl ResponseExt for reqwest::Response {
    fn display_body(&mut self) {
        println!("{}", self.text().unwrap());
    }

    fn display_headers(&self) {
        println!("// {:?} {}", self.version(), self.status());

        for (key, value) in self.headers().iter() {
            println!("// {}: {}", key, value.to_str().unwrap());
        }
    }
}

pub fn parse(
    filename: &str,
    params: &HashMap<&str, &str>,
    options: &napstruct::napoption::NapOptions,
) {
    let file = File::open(filename);
    let mut tmp = Vec::<String>::new();
    let mut cpt = 0;

    for line in BufReader::new(file.unwrap()).lines() {
        let current = line.unwrap();
        if current.starts_with('#') {
            if !tmp.is_empty() {
                cpt += 1;
                let mut request = napstruct::Request::from_vec(tmp);

                tmp = Vec::<String>::new();

                if !options.selecteds.contains(&cpt) {
                    continue;
                }
                if !request.is_empty() {
                    request.fix_params(&params);

                    let mut res = request.send();
                    res.display_body();
                    request.display();
                    res.display_headers();

                    if options.interval > time::Duration::from_millis(0) {
                        thread::sleep(options.interval);
                    }
                }
            }
        } else {
            tmp.push(current);
        }
    }
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

    for select in selected {
        no.add_selected(select);
    }

    // TODO function
    let mut params: HashMap<&str, &str> = HashMap::new();
    for current in matches.values_of("parameter").unwrap() {
        let tmp = current.split("=").collect::<Vec<&str>>();
        params.insert(tmp[0], tmp[1]);
    }

    let filename = matches.value_of("file").unwrap();
    parse(filename, &params, &no);
}
