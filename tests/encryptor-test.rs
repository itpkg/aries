// 
// extern crate rustc_serialize as serialize;
// extern crate rand;
// extern crate aries;
//
// use std::iter::repeat;
// use self::serialize::base64;
// use self::serialize::base64::ToBase64;
// use self::rand::{thread_rng, Rng};
// use aries::crypto::{Crypto, AesAndHmacCrypto};
//
// fn test_crypto<T: Crypto>(c: T) {
//     let msg = "Hello, Aries!!";
//     println!("Message: {}", msg);
//     let pwd = c.sum(msg.to_string());
//     println!("HMAC digest: {}", pwd);
//     assert!(c.check(msg.to_string(), pwd));
//
//     let (code, nonce) = c.encrypt(msg.to_string());
//     println!("AES digest: {}, {}",
//              nonce.to_base64(base64::STANDARD),
//              code);
//
//
// }
//
// #[test]
// fn test_aes_and_hmac_crypto() {
//     let mut hmac_key: Vec<u8> = repeat(0u8).take(32).collect();
//     thread_rng().fill_bytes(&mut hmac_key);
//     let mut aes_key: Vec<u8> = repeat(0u8).take(32).collect();
//     thread_rng().fill_bytes(&mut aes_key);
//
//     println!("HMAC key: {}", hmac_key.to_base64(base64::STANDARD));
//     println!("AES key: {}", aes_key.to_base64(base64::STANDARD));
//
//     let cyp = AesAndHmacCrypto::new(aes_key, hmac_key);
//     test_crypto(cyp);
// }
