extern crate aries;
extern crate postgres as pg;

use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;
use aries::orm::{Model, Dao, postgresql};


#[derive(Debug)]
struct Log {
    id: i64,
    message: &'static str,
}

impl Model for Log {
    fn queries(&self) -> HashMap<&'static str, &'static str> {
        let mut queries = HashMap::new();
        queries.insert("all", "SELECT * FROM logs");
        queries.insert("insert", "INSERT INTO logs(id, message) VALUES($1, $2)");
        queries.insert("now", "SELECT CURRENT_TIME, CURRENT_DATE");
        queries
    }
}

#[test]
fn test_orm_postgresql() {
    let l = Log {
        id: 1,
        message: "hello, aries!",
    };
    println!("item: {:?}", l);

    let args = BTreeMap::new();
    let dao = postgresql::Dao::new(l,
                                   "postgresql://postgres@localhost/aries_test",
                                   pg::SslMode::None)
        .unwrap();

    match dao.execute("now", args) {
        Ok(v) => println!("now: {}", v),
        Err(e) => println!("Error: {:?}", e),
    }

}
