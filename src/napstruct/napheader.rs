
#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub value: String,
}

impl Header {
    pub fn new(name: String, value: String) -> Header {
        Header { name, value }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_new() {
        let h = Header::new("name".to_string(), "value".to_string());
        assert_eq!(h.name, "name");
        assert_eq!(h.value, "value");
    }

}
