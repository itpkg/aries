use super::super::web;

pub struct Engine {

}

impl Engine {
    pub fn new() -> Engine {
        Engine {}
    }
}

impl web::engine::Engine for Engine {}
