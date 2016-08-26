pub mod redis;
pub mod postgresql;

use super::error::Result;


pub trait Connection {
    fn check(&self) -> bool;
    fn close(&self) -> Result<&'static str>;
}

pub trait Driver<C: Connection> {
    fn open(&self) -> Result<C>;
}


pub struct Pool<C: Connection, D: Driver<C>> {
    items: Vec<C>,
    driver: D,
}

impl<C: Connection, D: Driver<C>> Pool<C, D> {
    pub fn new(driver: D, len: usize) -> Result<Pool<C, D>> {
        let mut items = Vec::new();
        for _ in 0..len {
            match driver.open() {
                Ok(t) => items.push(t),
                Err(e) => return Err(e),
            }
        }
        Ok(Pool {
            items: items,
            driver: driver,
        })
    }
}

impl<C: Connection, D: Driver<C>> Pool<C, D> {
    pub fn get(&mut self) -> Result<C> {
        match self.items.pop() {
            Some(c) => {
                if c.check() {
                    Ok(c)
                } else {
                    self.driver.open()
                }
            }
            None => self.driver.open(),
        }
    }

    pub fn put(&mut self, c: C) {
        self.items.push(c);
    }

    pub fn release(&mut self) {
        while let Some(c) = self.items.pop() {
            match c.close() {
                Ok(msg) => info!("{}", msg),
                Err(err) => error!("{}", err),
            }
        }
    }
}
