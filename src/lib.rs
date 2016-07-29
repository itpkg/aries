pub mod crypto;
pub mod orm;
pub mod web;
pub mod inject;

#[derive(Debug)]
pub enum Error {
    Io(&'static str),
    Db(String),
    None,
}
