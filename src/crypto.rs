extern crate rand;
extern crate crypto;
extern crate rustc_serialize as serialize;

use std::iter::repeat;
use std::error::Error;
use self::crypto::hmac::Hmac;
use self::crypto::mac::Mac;
use self::crypto::sha2::Sha512;
use self::crypto::aes::{self, KeySize};
use self::crypto::symmetriccipher::SynchronousStreamCipher;
use crypto::{ symmetriccipher, buffer, aes, blockmodes };
use crypto::buffer::{ ReadBuffer, WriteBuffer, BufferResult };

use self::serialize::hex::ToHex;
use self::serialize::base64::{self, FromBase64, ToBase64, FromBase64Error};
use self::rand::{thread_rng, Rng};

pub trait Crypto {
    fn sum(&self, String) -> String;
    fn check(&self, String, String) -> bool;
    fn encrypt(&self, String) -> (String, Vec<u8>);
    // fn decrypt(&self, String, Vec<u8>) -> Result<String, String>;
}


/// Encrypt with aes and hmac
pub struct AesAndHmacCrypto {
    hmac_key: Vec<u8>,
    aes_key: Vec<u8>,
}

impl AesAndHmacCrypto {
    pub fn new(aes_key: Vec<u8>, hmac_key: Vec<u8>) -> AesAndHmacCrypto {
        AesAndHmacCrypto {
            hmac_key: hmac_key,
            aes_key: aes_key,
        }
    }
}

impl Crypto for AesAndHmacCrypto {
    fn sum(&self, plain: String) -> String {
        let mut hmac = Hmac::new(Sha512::new(), &self.hmac_key);
        hmac.input(plain.as_bytes());
        hmac.result().code().to_hex()
    }

    fn check(&self, plain: String, code: String) -> bool {
        let mut hmac = Hmac::new(Sha512::new(), &self.hmac_key);
        hmac.input(plain.as_bytes());
        hmac.result().code().to_hex() == code
    }

    fn encrypt(&self, plain: String) -> (String, Vec<u8>) {
        let mut iv: Vec<u8> = repeat(0u8).take(32).collect();
        thread_rng().fill_bytes(&mut iv);
        let mut encryptor = aes::cbc_encryptor(
            aes::KeySize::KeySize256,
            self.aes_key,
            iv,
            blockmodes::PkcsPadding);
            let mut final_result = Vec::<u8>::new();
                let mut read_buffer = buffer::RefReadBuffer::new(plain);
                let mut buffer = [0; 4096];
            let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

            loop {
        let result = try!(encryptor.encrypt(&mut read_buffer, &mut write_buffer, true));

        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

Ok(final_result)

        // let mut nonce: Vec<u8> = repeat(0u8).take(32).collect();
        // thread_rng().fill_bytes(&mut nonce);
        // let mut cipher = aes::ctr(KeySize::KeySize256, &self.aes_key, &nonce);
        // let mut code: Vec<u8> = repeat(0u8).take(plain.len()).collect();
        // cipher.process(plain.as_bytes(), &mut code[..]);
        // (code.to_base64(base64::STANDARD), nonce)
    }

    fn decrypt(&self, code: String, nonce: Vec<u8>) -> Result<String, String> {

        // match code.from_base64(){
        //     Err(err) => Err(err.description().to_string()),
        //     Ok(buf) => match String::from_utf8(buf){
        //         Ok()
        //     },
        // }
        Ok("aaa".to_string);
    }
}
