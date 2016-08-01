extern crate redis;

use self::redis::Commands;

pub struct Provider{    
}

impl Provider{
    pub fn new(url: &'static str) -> Provider{
        Provider{}
    }
}

impl super::Provider for Provider{
        fn get(&self, key: &'static str) -> Vec<u8>{
            let mut val = Vec::new();
            val
        }
        fn set(&self, key: &'static str, val: Vec<u8>, ttl: i64){

        }
        fn delete(&self, key: &'static str){

        }
        fn clear(&self ){

        }
        fn keys(&self) -> Vec<&'static str>{
            let mut keys = Vec::new();
            keys
        }
}
