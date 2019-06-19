use futures::{future, Future};
use hyper::Client;
use hyper::Request as hr;

use hyper_tls::HttpsConnector;

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
