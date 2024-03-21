
mod bitcoin_rpc_client;
mod routes;

//use bitcoin_rpc::BitcoinRPC;
//use bitcoin::Network;
//use std::error::Error;
mod wallet;
use actix_web::{App, HttpServer};
use routes::configure_routes;
use env_logger::Env;


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
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
     
    HttpServer::new(|| {
        App::new()
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

