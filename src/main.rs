#[macro_use]
extern crate clap;
extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::time;
use std::vec::Vec;

use clap::{App, Arg};

mod napstruct;

fn main() {
    color_backtrace::install();
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
                .multiple(true)
                .help("select only some requests as -s index1 -s index 2... start at 1")
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

    let selected = matches.values_of("select");
    match selected {
        Some(val) => {
            for current in val{
                let tmp = current.parse::<usize>();
                match tmp {
                    Ok(num) => { no.add_selected(num); }
                    _ => { }
                }
            }
        }
        None => { }
    }

    // TODO function
    let mut params: HashMap<String, String> = HashMap::new();
    let values = matches.values_of("parameter");
    match values {
	Some(val) => {
	    for current in val {
		let tmp = current.split("=").collect::<Vec<&str>>();
		params.insert(tmp[0].to_string(), tmp[1].to_string());
	    }
	}
	None => { }

    };

    let mut reader : Box<dyn BufRead> = match matches.value_of("file") {
        Some(filename) if filename != "-" => Box::new(BufReader::new(File::open(filename).unwrap())),
        _ => Box::new(BufReader::new(io::stdin())),
    };

    let parser = napstruct::parser::Parser::new();
    parser.run(&mut reader, &mut params, &no);
}
