
use actix_web::web;
use actix_web::HttpResponse;
use bitcoin::util::address::Address;
use bitcoin::secp256k1::{SecretKey, Secp256k1};
use bitcoin::secp256k1::PublicKey as SecpPublicKey;
use bitcoin::PublicKey;
use crate::wallet::WalletInfo;
use crate::wallet::{Wallet, ImportWalletInfo};
use crate::mnemonic::MnemonicPhrase;
use crate::libs::db_connection::DbConnection;
use log::debug;


// Handler function for creating a new wallet
async fn create_wallet(info: web::Json<WalletInfo> ) -> HttpResponse {

    let name = info.name.clone();
    let wallet = Wallet::new(&name);
    match wallet {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }

}
 
// Handler function for importing an existing wallet
async fn import_wallet(info: web::Json<ImportWalletInfo>) -> HttpResponse {
    let private_key = info.private_key.clone();
    let wallet = Wallet::import(&info.name, &private_key.to_string());
    match wallet {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub fn generate_keypair() -> (SecretKey, String, String) {
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
    (
        private_key,
        public_key.to_string(),
        address.to_string()
    )
}

// Handler function for generating a new mnemonic phrase
async fn generate_mnemonics (info: web::Json<MnemonicPhrase>) -> HttpResponse {
    let wallet_id = info.wallet_id;
    let mnemonic = MnemonicPhrase::new(wallet_id);
    match mnemonic {
        Ok(mnemonic) => HttpResponse::Ok().json(mnemonic),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
    
}

// Handler function for getting the mnemonic phrase for a wallet
async fn get_mnemonics(info: web::Json<MnemonicPhrase>) -> HttpResponse {
    let wallet_id = info.wallet_id;
    let db = DbConnection::new().expect("Failed to create DB connection");
    let mnemonic = MnemonicPhrase::get_wallet_mnemonic(wallet_id, &db);
    match mnemonic {
        Ok(mnemonic) => HttpResponse::Ok().json(mnemonic),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

// confirm Mnemonic phrase
async fn confirm_mnemonics(info: web::Json<MnemonicPhrase>) -> HttpResponse {
    let wallet_id = info.wallet_id;
    let phrase = info.phrase.clone();
    let db = DbConnection::new().expect("Failed to create DB connection");
    let mnemonic = MnemonicPhrase::confirm_phrase(wallet_id, &phrase, &db);
    match mnemonic {
        Ok(mnemonic) => HttpResponse::Ok().json(mnemonic),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
   
        .route("/create_wallet", web::post().to(create_wallet))
        .route("/import_wallet", web::post().to(import_wallet))
        .route("/generate_mnemonics", web::post().to(generate_mnemonics))
        .route("/get_mnemonics", web::post().to(get_mnemonics))
        .route("/confirm_mnemonics", web::post().to(confirm_mnemonics))

    );
}
