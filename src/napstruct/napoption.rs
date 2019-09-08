use std::time;

#[derive(Debug)]
pub struct NapOptions {
    pub interval: time::Duration,
    pub selecteds: Vec<usize>,
}

impl NapOptions {
    pub fn new() -> NapOptions {
        NapOptions {interval: time::Duration::from_millis(0),
                    selecteds: Vec::new()}
    }
    pub fn set_interval(&mut self, interval: time::Duration){
        self.interval = interval;
    }

    pub fn add_selected(&mut self, request_id: usize){
        self.selecteds.push(request_id);
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_new() {
        let no = NapOptions::new();
        assert_eq!(no.interval, time::Duration::from_millis(0));
        assert_eq!(no.selecteds.len(), 0);
    }

    #[test]
    fn test_set_interval() {
        let mut no = NapOptions::new();
        assert_eq!(no.interval, time::Duration::from_millis(0));
        no.set_interval(time::Duration::from_millis(4));
        assert_eq!(no.interval, time::Duration::from_millis(4));
    }

    #[test]
    fn test_add_selectef() {
        let mut no = NapOptions::new();
        assert_eq!(no.selecteds.len(), 0);
        no.add_selected(4);
        assert_eq!(no.selecteds.len(), 1);
    }
}
