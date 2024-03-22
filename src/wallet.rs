
use bitcoin::secp256k1::SecretKey;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use actix_web::Error;
use crate::routes::generate_keypair;
use mysql::params;
use crate::libs::db_connection::DbConnection;
use mysql::prelude::Queryable;



#[derive(Serialize, Deserialize)]
pub struct Wallet {
   pub id: Option<i32>,
   pub name: String,
    #[serde(serialize_with = "serialize_secret_key", deserialize_with = "deserialize_secret_key")]
   pub private_key: SecretKey,
   pub public_key : String,
   pub  address : String
}

impl Wallet {

    pub fn new(name: &str) -> Result<Self, Error> {
        
        let (private_key, public_key, address) = generate_keypair();

        let wallet = Self {
            id: Some(0),
            name: name.to_string(),
            private_key,
            public_key,
            address
        };

        let db =  DbConnection::new().map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

        // Save the wallet to the database
        wallet.save_to_db(&db)?;

        Ok(wallet)
    }



    fn save_to_db(&self, db: &DbConnection) -> Result<Wallet, Error> {
        // Get a connection from the database connection pool
        let mut conn = db.get_connection()
            .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    
        // Convert the private key to a string
        let private_key_str = self.private_key.to_string();
    
        // Execute the SQL query to insert the wallet into the database
        conn.exec_drop(
            r"INSERT INTO wallets (name, private_key, public_key, address)
            VALUES (:name, :private_key, :public_key, :address)",
            params! {
                "name" => &self.name,
                "private_key" => &private_key_str,
                "public_key" => &self.public_key,
                "address" => &self.address,
            },
        )
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    
        Ok(Wallet {
            id: self.id,
            name: self.name.clone(),
            private_key: self.private_key.clone(),
            public_key: self.public_key.clone(),
            address: self.address.clone(),
        })
    }
    

    pub fn import(name: &str, private_key_str: &str) -> Result<Self, Error> {
        // Convert the private key string to a SecretKey
        let _private_key = SecretKey::from_str(private_key_str)
            .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    
        // Create a new DB connection
        let db = DbConnection::new()
            .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    
        // Get a connection from the database connection pool
        let mut conn = db.get_connection()
            .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    
        // Execute a SQL query to fetch the wallet based on the provided name and private key
        let row_opt = conn.exec_first(
            r"SELECT name, private_key, public_key, address
            FROM wallets
            WHERE name = :name AND private_key = :private
            LIMIT 1",
            params! {
                "name" => name,
                "private" => private_key_str,
            },
        )
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    
        // Check if a row was found
        if let Some(row) = row_opt {
            // Extract the wallet data from the row
            let ( id, name, private_key_str, public_key, address): (i32, String, String, String, String) =
                mysql::from_row(row);
            // Convert the private key string to a SecretKey
            let private_key = SecretKey::from_str(&private_key_str)
                .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    
            // Create the wallet instance
            let wallet = Self {
                id: Some(id),
                name: name,
                private_key: private_key,
                public_key: public_key,
                address: address,
            };
    
            Ok(wallet)
        } else {
            // Return an error indicating that the wallet was not found
            Err(actix_web::error::ErrorNotFound("Wallet not found"))
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

