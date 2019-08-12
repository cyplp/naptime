use reqwest;

pub mod napheader;

#[derive(Debug)]
pub struct Request {
    pub verb: String,
    pub url: String,
    pub headers: Vec<napheader::Header>,
    pub body: String,
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

    pub fn add_header(&mut self, header: napheader::Header) {
        self.headers.push(header);
    }

    pub fn add_body(&mut self, body: String) {
        self.body = body;
    }

    pub fn is_empty(&self) -> bool {
        self.verb.is_empty() || self.url.is_empty()
    }

    fn is_header(line: &str) -> bool {
        line.contains(": ")
    }

    pub fn from_vec(buffer: Vec<String>) -> Request {
        let first = buffer[0].split(' ').collect::<Vec<&str>>();

        let mut request = Request::new(first[0].to_string(), first[1..].join(" "));

        let mut body = false;
        let mut tmp: Vec<String> = Vec::new();

        for line in buffer.iter().skip(1) {
            if !body {
                if Request::is_header(&line) {
                    let headers = line.split(": ").collect::<Vec<&str>>();
                    request.add_header(napheader::Header::new(headers[0].to_string(), headers[1..].join(": ")));
                } else {
                    body = true;
                }
            }

            if body {
                tmp.push(line.to_string());
            }
        }
        if !tmp.is_empty() {
            request.add_body(tmp.join("\n"));
        }
        request
    }

    pub fn run(&self) -> reqwest::Response{
        // TODO : refactor that
        let client = reqwest::Client::new();
        let mut req = client.request(reqwest::Method::from_bytes(self.verb.as_bytes()).unwrap(),
                                     self.url.as_str());

        for header in &self.headers {
           req = req.header(header.name.as_str(), header.value.as_str());
        }

        req = req.body(self.body.clone());
        let res = req.send().unwrap();

        res
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_new() {
        let r = Request::new("POST".to_string(), "https://some.url".to_string());
        assert_eq!(r.verb, "POST");
        assert_eq!(r.url, "https://some.url");
        assert_eq!(r.headers.is_empty(), true);
        assert_eq!(r.body.is_empty(), true);
    }

    #[test]
    fn test_add_header() {
        let mut r = Request::new("POST".to_string(), "https://some.url".to_string());
        assert_eq!(r.headers.is_empty(), true);
        r.add_header(napheader::Header::new("some".to_string(), "header".to_string()));
        assert_eq!(r.headers.is_empty(), false);
        assert_eq!(r.headers[0], napheader::Header::new("some".to_string(), "header".to_string()));
    }

    #[test]
    fn test_add_body() {
        let mut r = Request::new("POST".to_string(), "https://some.url".to_string());
        assert_eq!(r.body.is_empty(), true);
        r.add_body("body".to_string());
        assert_eq!(r.body.is_empty(), false);
        assert_eq!(r.body, "body");
    }

    #[test]
    fn test_is_empty() {
        let r = Request::new("POST".to_string(), "https://some.url".to_string());
        assert_eq!(r.is_empty(), false);

        let r = Request::new("".to_string(), "https://some.url".to_string());
        assert_eq!(r.is_empty(), true);

        let r = Request::new("POST".to_string(), "".to_string());
        assert_eq!(r.is_empty(), true);
    }

    #[test]
    fn test_is_header() {
        assert_eq!(Request::is_header("Content: application/json"), true);
        assert_eq!(Request::is_header("Content :application/json"), false);
    }

    #[test]
    fn test_from_vec() {
        let v = vec!("POST https://some.url".to_string(),
                     "SomeHeader: SomeValue".to_string(),
                     "{\"some\": \"body\",".to_string(),
                     "\"other\":\"value\"}".to_string());
        let r = Request::from_vec(v);
        assert_eq!(r.verb, "POST");
        assert_eq!(r.is_empty(), false);

        let v = vec!("".to_string());
        let r = Request::from_vec(v);
        assert_eq!(r.is_empty(), true);
    }
}
