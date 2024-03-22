
mod bitcoin_rpc_client;
mod routes;

//use bitcoin_rpc::BitcoinRPC;
//use bitcoin::Network;
//use std::error::Error;
mod wallet;
pub mod libs;
pub mod mnemonic;
use actix_web::{App, HttpServer};
use routes::configure_routes;
use libs::db_connection::DbConnection;


/* fn main() -> Result<(), Box<dyn Error>> {
    
    let rpc_user = "your_rpc_username";
    let rpc_pass = "your_rpc_password";
    let rpc_url = "http://localhost:8332"; // Replace with your Bitcoin node's RPC URL

    // Initialize BitcoinRPC instance
    let bitcoin_rpc = BitcoinRPC::new(rpc_url, rpc_user, rpc_pass, Network::Bitcoin)?;

    // Example: Get the blockchain info
    let blockchain_info = bitcoin_rpc.get_blockchain_info()?;
    println!("Blockchain Info: {:?}", blockchain_info);

    Ok(())
}
 */

 #[actix_web::main]
  async fn main() -> std::io::Result<()> {
    env_logger::init();
    println!("Starting server at: http://127.0.0.1:8080");
    DbConnection::new().expect("Failed to create DB connection");


     
    HttpServer::new(|| {
        App::new()
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

