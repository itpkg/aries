extern crate hyper;

use rustc_serialize::Encodable;

pub struct Render<T: Encodable> {
    body: T,
}

impl<T: Encodable> Render<T> {
    pub fn json() {}
    pub fn xml() {}
    pub fn bytes() {}
    pub fn file() {}
    pub fn html() {}
}
