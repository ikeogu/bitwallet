use bitcoin::secp256k1::{Secp256k1, SecretKey};
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use actix_web::Error;
use rand::RngCore;
use mysql::params;



#[derive(Serialize, Deserialize)]
pub struct Wallet {
    name: String,
    #[serde(serialize_with = "serialize_secret_key", deserialize_with = "deserialize_secret_key")]
    private_key: SecretKey,
}

impl Wallet {

    pub fn new(name: &str) -> Result<Self, Error> {
        // Generate a new random private key
        let secp = Secp256k1::new();
        let mut rng = rand::thread_rng();
        let mut private_key_bytes = [0u8; 32];
        rng.fill_bytes(&mut private_key_bytes); 

        // Create the wallet
        let private_key = SecretKey::from_slice(&private_key_bytes)
            .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

        let wallet = Self {
            name: name.to_string(),
            private_key,
        };

        let db = Database::new().map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

        // Save the wallet to the database
        wallet.save_to_db(db)?;

        Ok(wallet)
    }


    pub fn import(name: &str, private_key_str: &str) -> Result<Self, bitcoin::secp256k1::Error> {
        let private_key = SecretKey::from_str(private_key_str)?;

        let wallet = Self {
            name: name.to_string(),
            private_key,
        };

        let db = Database::new().map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

        // Save the wallet to the database
        wallet.save_to_db(db)?;

        Ok(wallet)
    }

    fn save_to_db(&self, db: &Database) -> Result<(), Error> {
        let conn = db.get_connection()?;
        let private_key_str = self.private_key.to_string(); // Convert private key to string
        
        conn.exec_drop(
            "INSERT INTO wallets (name, private_key) VALUES (?, ?)",
            (self.name.clone(), private_key_str), // Pass parameters as a tuple
        )?;
        
        Ok(())
    }

    fn load_from_db(name: &str, db: &Database) -> Result<Self, Error> {
        let conn = db.get_connection()?;
        
        let mut stmt = conn.prepare("SELECT name, private_key FROM wallets WHERE name = ?")?;
        let row_opt = stmt.execute(params![name])?.next();
        
        if let Some(row) = row_opt {
            let (name, private_key_str): (String, String) = mysql::from_row(row?);
            let private_key = SecretKey::from_str(&private_key_str)
                .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

            Ok(Self {
                name,
                private_key,
            })
        } else {
            Err(actix_web::error::ErrorBadRequest("Wallet not found"))
        }
    }
}



#[derive(Serialize, Deserialize)]
pub struct WalletInfo {
    pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct ImportWalletInfo {
    pub name: String,
    #[serde(serialize_with = "serialize_secret_key", deserialize_with = "deserialize_secret_key")]
    pub private_key: SecretKey,
}

fn serialize_secret_key<S>(secret_key: &SecretKey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let hex_string = format!("{:02x}", secret_key);
    serializer.serialize_str(&hex_string)
}

fn deserialize_secret_key<'de, D>(deserializer: D) -> Result<SecretKey, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let hex_string = String::deserialize(deserializer)?;
    if hex_string.is_empty() {
        // Handle the case where the private key field is missing or empty
        // You can return an error or a default value here depending on your application logic
        return Err(serde::de::Error::custom("Missing or empty private key"));
    }
    SecretKey::from_str(&hex_string).map_err(serde::de::Error::custom)
}

