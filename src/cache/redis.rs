extern crate redis;

//use self::redis::Commands;

pub struct Provider {
}

impl Provider {
    pub fn new(_: &'static str) -> Provider {
        Provider {}
    }
}

impl super::Provider for Provider {
    fn get(&self, _: &'static str) -> Vec<u8> {
        //TODO
        let val = Vec::new();
        val
    }
    fn set(&self, _: &'static str, _: Vec<u8>, _: i64) {
        //TODO
    }
    fn delete(&self, _: &'static str) {
        //TODO
    }
    fn clear(&self) {}
    fn keys(&self) -> Vec<&'static str> {
        //TODO
        let keys = Vec::new();
        keys
    }
}
