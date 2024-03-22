/* use bitcoincore_rpc::{Auth, Client, RpcApi};
//use bitcoin::Network;
use std::error::Error;

pub struct BitcoinRPC {
    rpc_client: Client,
}

impl BitcoinRPC {
    pub fn new(rpc_url: &str, rpc_user: &str, rpc_pass: &str, network: bitcoincore_rpc::bitcoin::Network) -> Result<Self, Box<dyn Error>> {
        let rpc_client = Client::new(
            rpc_url,
            Auth::UserPass(rpc_user.to_string(), rpc_pass.to_string()),
        )?;
        Ok(Self { rpc_client })
    }

    pub fn get_blockchain_info(&self) -> Result<bitcoincore_rpc::json::GetBlockchainInfoResult, bitcoincore_rpc::Error> {
        self.rpc_client.get_blockchain_info()
    }
}
 */