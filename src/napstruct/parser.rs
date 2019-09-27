use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{thread, time};
use regex::Regex;

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
enum LineType {
    Comment,
//    Target,
    Header,
//    Empty,
    Body,
    Param,
    DynParam,
}

#[derive(Debug)]
pub struct Parser<'a> {
    filename: &'a str,
}

impl Parser<'_> {
    pub fn new(filename: &str) -> Parser {
        Parser {filename: filename}
    }

    fn type_line(&self, line: &str) -> LineType{
        if line.starts_with('#') {
            LineType::Comment
        }
        else if self.is_header(line){
            LineType::Header
        }
        else if self.is_param(line){
            LineType::Param
        }
        else if self.is_dyn_param(line) {
            LineType::DynParam
        }
        else {LineType::Body}
    }

    fn is_header(&self, line: &str) -> bool {
        // TODO refactor this
        let is_header: Regex = Regex::new(r"^[\w-]+: .*$").unwrap();
        is_header.is_match(line)
    }

    fn is_param(&self, line: &str) -> bool {
        // TODO refactor this
        let is_param = Regex::new(r":\w+ = .*$").unwrap();
        is_param.is_match(line)
    }

    fn is_dyn_param(&self, line: &str) -> bool {
        // TODO refactor this
        let is_dyn_param = Regex::new(r":\w+ := .*$").unwrap();
        is_dyn_param.is_match(line)
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
            match self.type_line(current.as_str())
            {
                LineType::Comment => {
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
                }
                _ => {
                    tmp.push(current);
                }
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

    #[test]
    fn test_is_header() {
        let p = Parser::new("some/file");

        assert_eq!(p.is_header("Content: application/json"), true);
        assert_eq!(p.is_header("Content :application/json"), false);
        assert_eq!(p.is_header("http://some.url:80"), false);
    }

    #[test]
    fn test_is_param() {
        let p = Parser::new("some/file");

        let line = ":some = param";
        assert_eq!(p.is_param(line), true);

        let line = ":some := dyn param";
        assert_eq!(p.is_param(line), false);

        let line = "POST http://some.url";
        assert_eq!(p.is_param(line), false);
    }

    #[test]
    fn test_is_dyn_param() {
        let p = Parser::new("some/file");

        let line = ":some = param";
        assert_eq!(p.is_dyn_param(line), false);

        let line = ":some := dyn param";
        assert_eq!(p.is_dyn_param(line), true);

        let line = "POST http://some.url";
        assert_eq!(p.is_dyn_param(line), false);
    }
}
