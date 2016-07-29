extern crate rand;
extern crate uuid;

use std::iter::repeat;
use self::rand::Rng;

pub fn gen_uuid_v4() -> String{
    return uuid::Uuid::new_v4().to_string();
}

pub fn gen_string(n :usize) -> String{
    return rand::thread_rng()
        .gen_ascii_chars()
        .take(n)
        .collect::<String>();

}

pub fn gen_bytes(n :usize) -> Vec<u8>{
    let mut buf: Vec<u8> = repeat(0u8).take(n).collect();
    rand::thread_rng().fill_bytes(&mut buf);
    buf
}
