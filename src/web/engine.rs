use super::super::orm;
use super::router::Router;

pub trait Engine {
    fn mount<T: Router>(T);
    fn init<T: orm::Db>(T);
}
