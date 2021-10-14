pub mod parser;
pub mod error;
pub mod color;

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

pub fn encrypt_sw_file_to_file(file: &Path, out: Option<&Path>) -> SWRSResult<()> {
    let output_file = out.unwrap_or_else(|| file);

    let input_plaintext = std::fs::read(file);
    if input_plaintext.is_err() {
        return Err(SWRSError::IOError(input_plaintext.unwrap_err()));
    }

    let output_ciphertext = encrypt_sw(&input_plaintext.unwrap())?;

    let write_result = std::fs::write(output_file, output_ciphertext);
    if write_result.is_err() {
        Err(SWRSError::IOError(write_result.unwrap_err()))
    } else {
        Ok(())
    }
}

pub fn encrypt_sw_file(file: &Path) -> SWRSResult<Vec<u8>> {
    let input_plaintext = std::fs::read(file);
    if input_plaintext.is_err() {
        return Err(SWRSError::IOError(input_plaintext.unwrap_err()));
    }

    let output_ciphertext = encrypt_sw(&input_plaintext.unwrap())?;
    Ok(output_ciphertext)
}

pub fn encrypt_sw(data: &Vec<u8>) -> SWRSResult<Vec<u8>> {
    let cipher = Aes128Cbc::new_from_slices(KEY.as_ref(), KEY.as_ref())
        .expect("Failed to create the AES key");

    Ok(cipher.encrypt_vec(&*data))
}