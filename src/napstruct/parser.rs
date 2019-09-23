use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{thread, time};

use crate::napstruct;

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

#[derive(Debug)]
pub struct Parser<'a> {
    filename: &'a str,
}

impl Parser<'_> {
    pub fn new(filename: &str) -> Parser {
        Parser {filename: filename}
    }

    pub fn run(
        &self,
        params: &HashMap<&str, &str>,
        options: &napstruct::napoption::NapOptions,
    ) {
        let file = File::open(self.filename);
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
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_new() {
        let p = Parser::new("some/file");
        assert_eq!(p.filename, "some/file");
    }
}
