//! Storagw
//! It implements logic to store data on disk.

use std::error::Error;
use std::fs::File;

use crate::wallet::Wallet;

#[cfg(test)]
#[path = "test/storage_test.rs"]
mod storage_test;

#[derive(Debug)]
pub struct Storage {}

impl Storage {
    /// Saves wallet data to file.
    pub fn save(dest_path: String, wallet: &Wallet) -> Result<(), Box<dyn Error>> {
        let data_file = File::create(dest_path)?;

        serde_cbor::to_writer(data_file, &wallet)?;

        Ok(())
    }

    /// Loads wallet data from file.
    pub fn load(src_path: String) -> Result<Wallet, Box<dyn Error>> {
        let read_file = File::open(src_path)?;

        // NOTE: it doesn't work using `?` syntax, getting this errror:
        // the trait `wallet::_IMPL_DESERIALIZE_FOR_OperationTypes::_serde::Deserialize<'_>` is not implemented for `dyn std::error::Error`
        match serde_cbor::from_reader(read_file) {
            Ok(w) => Ok(w),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn load_or_new(src_path: String) -> Wallet {
        match Storage::load(src_path) {
            Ok(w) => w,
            Err(_e) => Wallet::new(String::from("default"), 0),
        }
    }
}
