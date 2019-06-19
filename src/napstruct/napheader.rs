
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

impl PartialEq for Header {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
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

    #[test]
    fn test_eq() {
        let h1 = Header::new("name".to_string(), "value".to_string());
        let h2 = Header::new("name".to_string(), "value".to_string());
        let h3 = Header::new("name3".to_string(), "value".to_string());
        let h4 = Header::new("name".to_string(), "value3".to_string());

        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
        assert_ne!(h1, h4);
    }

}
