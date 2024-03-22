extern crate bip39;
extern crate rand;

use actix_web::error::Error;
use bip39::Mnemonic;
use rand::rngs::OsRng;
use rand::RngCore;
use crate::libs::db_connection::DbConnection;
use mysql::prelude::*;
use serde::{Serialize, Deserialize};
use mysql::params;

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub struct MnemonicPhrase {
    pub phrase: String,
   // #[serde[serialize_with = "serialize_mnemonic", deserialize_with = "deserialize_mnemonic"]]
    pub wallet_id: i32, // Reference to the wallet
}

impl MnemonicPhrase {
    pub fn new(wallet_id: i32) -> Result<Self, Error> {
        let phrase = MnemonicPhrase::generate_mnemonic();
        let mnemonic_phrase = Self {
            phrase: phrase.clone(),
            wallet_id,
        };

        let db = DbConnection::new().map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
        mnemonic_phrase.save_to_db(&db)?;

        Ok(mnemonic_phrase)
    }

    pub fn generate_mnemonic() -> String {
        let mut entropy = [0u8; 32];
        OsRng.fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy(&entropy)
            .expect("Failed to generate mnemonic");
        mnemonic.to_string()
    }

    pub fn save_to_db(&self, db: &DbConnection) -> Result<(), Error> {
        let mut conn = db.get_connection()
            .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

        conn.exec_drop(
            r"INSERT INTO mnemonics (phrase, wallet_id)
            VALUES (:phrase, :wallet_id)",
            params! {
                "phrase" => &self.phrase,
                "wallet_id" => self.wallet_id,
            },
        ).map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

        Ok(())
    }

    pub fn get_wallet_mnemonic(wallet_id: i32, db: &DbConnection) -> Result<MnemonicPhrase, Error> {
        let mut conn = db.get_connection()
            .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

        let result = conn.exec_first::<(String, i32), _, _>(
            r"SELECT phrase, wallet_id FROM mnemonics WHERE wallet_id = :wallet_id",
            params! {
                "wallet_id" => wallet_id,
            },
        ).map_err(|err| {
            if let mysql::error::Error::MySqlError(mysql_err) = err {
                if mysql_err.code == 0 {
                    actix_web::error::ErrorInternalServerError("Mnemonic not found for the wallet")
                } else {
                    actix_web::error::ErrorInternalServerError(mysql_err.to_string())
                }
            } else {
                actix_web::error::ErrorInternalServerError(err.to_string())
            }
        })?;

        match result {
            Some((phrase, wallet_id)) => Ok(MnemonicPhrase { phrase, wallet_id }),
            None => Err(actix_web::error::ErrorInternalServerError("Mnemonic not found for the wallet")),
        }
    }

    pub fn confirm_phrase(wallet_id: i32, phrase: &str, db: &DbConnection) -> Result<bool, Error> {
        let mnemonic = MnemonicPhrase::get_wallet_mnemonic(wallet_id, db)?;

        Ok(mnemonic.phrase == phrase)
    }
}

