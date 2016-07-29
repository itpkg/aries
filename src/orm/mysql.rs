use std::collections::btree_map::BTreeMap;
use super::super::Error;

pub struct Dao {

}

impl super::Dao for Dao {
    fn execute<T: super::Model>(&self,
                                _: T,
                                _: &str,
                                _: BTreeMap<&'static str, super::Type>)
                                -> Result<i64, Error> {
        Err(Error::None)
    }
}
