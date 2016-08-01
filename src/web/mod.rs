extern crate hyper;

use self::hyper::server::{Request, Response};
use super::orm;

pub mod binding;
pub mod context;
pub mod render;
pub mod router;
pub mod validator;

pub trait Engine {
    fn mount<T: Router>(T);
    fn init<T: orm::Db>(T);
}

pub trait Router {
    fn get(&'static str, Request, Response);
    fn post(&'static str, Request, Response);
    fn put(&'static str, Request, Response);
    fn patch(&'static str, Request, Response);
    fn delete(&'static str, Request, Response);
}
