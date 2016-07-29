extern crate aries;
extern crate rustc_serialize as serialize;

use aries::crypto::random;
use self::serialize::base64::{self, ToBase64};

#[test]
fn test_crypto_random_string() {
    let s = random::gen_string(16);
    println!("random string: {}", s);
}


#[test]
fn test_crypto_random_bytes() {
    let b = random::gen_bytes(32);
    println!("random bytes: {}", b.to_base64(base64::STANDARD));
}


#[test]
fn test_crypto_random_uuid() {
    let s = random::gen_uuid_v4();
    println!("uuid v4: {}", s);
}
