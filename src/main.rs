#[macro_use]
extern crate clap;
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::thread;
use std::time;
use std::vec::Vec;

use clap::Parser;

use crate::napstruct::parser::ResponseExt;

mod napstruct;

#[derive(Parser, Debug)]
#[ clap(author, version, about, long_about = None)]
struct Args {
    /// File containing requests with restclient.el format
    #[clap(short, long, default_value = "-")]
    file: String,

    /// interval between two requests in milliseconds
    #[clap(short, long, default_value_t = 0)]
    interval: u64,

    /// select only some requests as -s index1 -s index 2... start at 1
    #[clap(short, long)]
    select: Vec<usize>,

    /// set parameters
    #[clap(short, long)]
    parameters: Vec<String>,
}

fn main() {
    let matches = Args::parse();
    let mut no = napstruct::napoption::NapOptions::new();

    let interval = time::Duration::from_millis(matches.interval);
    no.set_interval(interval);

    for current in matches.select {
        no.add_selected(current);
    }

    // TODO function
    let mut params: HashMap<String, String> = HashMap::new();

    let values = matches.parameters;
    for current in values {
        let tmp = current.split("=").collect::<Vec<&str>>();
        params.insert(tmp[0].to_string(), tmp[1].to_string());
    }

    let mut reader: Box<dyn BufRead> = match matches.file {
        filename if filename != "-" => Box::new(BufReader::new(File::open(filename).unwrap())),
        _ => Box::new(BufReader::new(io::stdin())),
    };

    let parser = napstruct::parser::Parser::new();
    let mut reqs = parser.run(&mut reader, &mut params);

    for (i, r) in reqs.iter_mut().enumerate() {
        if !no.selecteds.is_empty() && !no.selecteds.contains(&i) {
            continue;
        }

        if r.is_err() {
            println!("Cannot parse request {}", i);
            continue;
        }

        let req = r.as_mut().unwrap();

        if !req.is_empty() {
            req.fix_params(&params);

            let mut res = req.send();
            res.display_body();
            req.display();
            res.display_headers();

            if no.interval > time::Duration::from_millis(0) {
                thread::sleep(no.interval);
            }
        }
    }
}
