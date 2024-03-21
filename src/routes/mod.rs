

use actix_web::web;

use actix_web::HttpResponse;
use actix_web::Responder;
use bitcoin::util::address::Address;
use bitcoin::secp256k1::{SecretKey, Secp256k1};
use serde_json::json;
use bitcoin::secp256k1::PublicKey as SecpPublicKey;
use bitcoin::PublicKey;
use crate::wallet::WalletInfo;
use crate::wallet::{Wallet, ImportWalletInfo};
use log::info;
use crate::db_connection::DBConnection;
use crate::db_connection::DBConnection;




// Handler function for creating a new wallet
async fn create_wallet(info: web::Json<WalletInfo> ) -> HttpResponse {

// ...

let db_connection = DBConnection::new().expect("Failed to create DB connection");
// ...

let db_connection = DBConnection::new().expect("Failed to create DB connection");
   let db_connection = crate::db_connection::DBConnection::new().expect("Failed to create DB connection");

   // Use the connection to get a pooled connection
   match db_connection.get_connection() {
       Ok(conn) => {
           // Connection successfully acquired, perform operations with the connection
       }
       Err(err) => {
           // Handle connection error
           eprintln!("Failed to get database connection: {}", err);
       }
   }

    match Wallet::new(&info.name) {
        Ok(wallet) => {
            // Save the wallet to the database
            match db.get_connection() {
                Ok(connection) => {
                    // Insert the wallet into the database
                    match insert_wallet(&connection, &wallet) {
                        Ok(_) => {
                            // Log success message
                            info!("Wallet created successfully: {:?}", wallet);

                            // Return the wallet in the HTTP response
                            HttpResponse::Ok().json(wallet)
                        }
                        Err(error) => {
                            // Log database insertion error
                            
                            // Return internal server error response
                            HttpResponse::InternalServerError().json(json!({
                                "error": "Failed to create wallet",
                                "details": "Error inserting wallet into database",
                            }))
                        }
                    }
                }
                Err(error) => {
                    // Log database connection error
                    //error!("Failed to get database connection: {}", error);

                    // Return internal server error response
                    HttpResponse::InternalServerError().json(json!({
                        "error": "Failed to create wallet",
                        "details": "Error connecting to database",
                    }))
                }
            }
        }
        Err(error) => {
            // Log wallet creation error
           
            // Return bad request response indicating invalid input
            HttpResponse::BadRequest().json(json!({
                "error": "Invalid input",
                "details": error.to_string(),
            }))
        }
    }
}
 // 
// Handler function for importing an existing wallet
async fn import_wallet(info: web::Json<ImportWalletInfo>) -> HttpResponse {
    let private_key = info.private_key.clone();
    let wallet = Wallet::import(&info.name, &private_key.to_string());
    match wallet {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn generate_keypair() -> impl Responder {
    // Generate a new random private key
    let secp = Secp256k1::new();
    let private_key_bytes: [u8; 32] = rand::random(); // Generate 32 random bytes
    let private_key = SecretKey::from_slice(&private_key_bytes)
        .expect("Failed to generate a new private key");

   // Derive the public key from the private key
   let public_key = SecpPublicKey::from_secret_key(&secp, &private_key);

   // Convert the public key to the bitcoin::PublicKey type
   let bitcoin_public_key = PublicKey::from_slice(public_key.serialize().as_ref())
       .expect("Failed to convert public key");


    // Derive the address from the public key
    let address = Address::p2pkh(&bitcoin_public_key, bitcoin::Network::Bitcoin);

    // Return the private key, public key, and address
    HttpResponse::Ok().json(json!({
        "private_key": private_key.to_string(),
        "public_key": public_key.to_string(),
        "address": address.to_string(),
    }))
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .route("/generate_keypair", web::get().to(generate_keypair))
        .route("/create_wallet", web::post().to(create_wallet))
        .route("/import_wallet", web::post().to(import_wallet))

    );
}
