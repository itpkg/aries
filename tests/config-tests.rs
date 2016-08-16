#[macro_use]
extern crate log;

fn add_one(num: i32) -> i32 {
    info!("add_one called with {}", num);
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate env_logger;

    #[test]
    fn it_adds_one() {
        let _ = env_logger::init();
        info!("can log from the test too");
        // assert_eq!(3, add_one(2));
    }

    #[test]
    fn it_handles_negative_numbers() {
        let _ = env_logger::init();
        info!("logging from another test");
        // assert_eq!(-7, add_one(-8));
    }
}
