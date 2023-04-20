pub mod parser;
pub mod color;

#[cfg(feature = "api")]
pub mod api;

pub(crate) mod util;

use std::io;
/// We use [`ritelinked::LinkedHashMap`] to preserve insertion order while having a key-value pair
/// storage mechanism
pub use ritelinked::LinkedHashMap;

use std::path::Path;
use std::string::FromUtf8Error;
use block_modes::{Cbc, BlockMode, BlockModeError};
use block_modes::block_padding::Pkcs7;
use aes::Aes128;
use thiserror::Error;

const KEY: &str = "sketchwaresecure";

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub fn decrypt_sw_file(file: &Path) -> Result<Vec<u8>, CryptoError> {
    let data = std::fs::read(file).map_err(CryptoError::IOError)?;

    decrypt_sw_encrypted(&data)
}

pub fn decrypt_sw_encrypted(data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let cipher = Aes128Cbc::new_from_slices(KEY.as_ref(), KEY.as_ref())
        .expect("Failed to create the AES key");

    let mut buffer: Vec<u8> = data.clone().to_vec();

    Ok(Vec::from(
        cipher.decrypt(&mut buffer)
            .map_err(CryptoError::DecryptionError)?
    ))
}

pub fn encrypt_sw_file_to_file(file: &Path, out: Option<&Path>) -> Result<(), CryptoError> {
    let output_file = out.unwrap_or_else(|| file);

    let input_plaintext = std::fs::read(file).map_err(CryptoError::IOError)?;
    let output_ciphertext = encrypt_sw(&input_plaintext);

    std::fs::write(output_file, output_ciphertext)
        .map_err(CryptoError::IOError)?;

    Ok(())
}

pub fn encrypt_sw_file(file: &Path) -> Result<Vec<u8>, CryptoError> {
    let input_plaintext = std::fs::read(file).map_err(CryptoError::IOError)?;

    let output_ciphertext = encrypt_sw(&input_plaintext);
    Ok(output_ciphertext)
}

pub fn encrypt_sw(data: &[u8]) -> Vec<u8> {
    let cipher = Aes128Cbc::new_from_slices(KEY.as_ref(), KEY.as_ref())
        .expect("Failed to create the AES key");

    cipher.encrypt_vec(&*data)
}

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("io error")]
    IOError(#[from] io::Error),

    #[error("decryption error")]
    DecryptionError(#[from] BlockModeError),

    #[error("couldn't convert binaries into utf8")]
    FromUtf8Error(#[from] FromUtf8Error)
}