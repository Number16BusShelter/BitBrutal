extern crate aes;
extern crate hex;
extern crate sha2;
extern crate openssl;

use openssl::symm::{decrypt, Cipher};
use sha2::{Digest, Sha512};

use crate::core::components::HashComponents;

fn calculate_sha512(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn decrypt_with_pkcs7_unpadding(cipher: Cipher, key: &[u8], iv: Option<&[u8]>, data: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
    decrypt(cipher, key, iv, data)
}

pub fn check<'a>(passphrase: &str, components: &HashComponents) -> bool {
    let temp = hex::encode(passphrase.as_bytes().to_vec()) + &components.salt;
    let mut pass_to_hash = hex::decode(temp).unwrap();

    for _ in 0..components.iterations_count {
        pass_to_hash = calculate_sha512(&pass_to_hash);
    }

    let key = &pass_to_hash[0..32];
    let iv = &pass_to_hash[32..48];

    match decrypt_with_pkcs7_unpadding(Cipher::aes_256_cbc(), key, Some(iv), &hex::decode(&components.master_key).unwrap()) {
        Ok(_) => true,
        Err(_) => false
    }
}