pub mod error;

use crate::error::{SWRSResult, SWRSError};
use std::path::Path;
use block_modes::{Cbc, BlockMode};
use block_modes::block_padding::Pkcs7;
use aes::Aes128;

const KEY: &str = "sketchwaresecure";

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub fn decrypt_sw_file(file: &Path) -> SWRSResult<Vec<u8>> {
    let data = std::fs::read(file);
    if data.is_err() {
        return SWRSResult::Err(SWRSError::IOError(data.unwrap_err()));
    }

    decrypt_sw_encrypted(&data.unwrap())
}

pub fn decrypt_sw_encrypted(data: &Vec<u8>) -> SWRSResult<Vec<u8>> {
    let cipher = Aes128Cbc::new_from_slices(KEY.as_ref(), KEY.as_ref())
        .expect("Failed to create the AES key");

    let mut buffer: Vec<u8> = data.clone().to_vec();
    let plaintext_res = cipher.decrypt(&mut buffer);

    if plaintext_res.is_err() {
        Err(SWRSError::DecryptionError(plaintext_res.unwrap_err()))
    } else {
        Ok(Vec::from(plaintext_res.unwrap()))
    }
}
