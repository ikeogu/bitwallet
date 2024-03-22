use mysql::{Pool, PooledConn, Opts, Error};
use mysql::prelude::*;
use std::env;


pub struct DbConnection {
    pool: Pool,
}

impl DbConnection {
   
  pub fn new() -> Result<Self, Error> {
    dotenv::dotenv().ok();
    let opts = Opts::from_url(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))?;
    let pool = Pool::new(opts)?;

    let db_connection = Self { pool };

      // Execute table creation functions
      db_connection.create_wallet_table()?;
      db_connection.create_transaction_table()?;
      db_connection.create_mnemonic_table()?;
      
      Ok(db_connection)
  
  }

  pub fn execute_query(&self, query: &str) -> Result<(), Error> {
      let mut conn = self.pool.get_conn()?;
      conn.query_drop(query)?;
      Ok(())
  }

  pub fn create_wallet_table(&self) -> Result<(), Error> {
      self.execute_query(r"CREATE TABLE IF NOT EXISTS wallets (
          id INT AUTO_INCREMENT PRIMARY KEY,
          name VARCHAR(50) NOT NULL UNIQUE,
          private_key VARCHAR(255) NOT NULL UNIQUE,
          public_key VARCHAR(255) NOT NULL UNIQUE,
          address VARCHAR(255) NOT NULL UNIQUE,
          created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
      )")
  }

  pub fn create_transaction_table(&self) -> Result<(), Error> {
      self.execute_query(r"CREATE TABLE IF NOT EXISTS transactions (
          id INT AUTO_INCREMENT PRIMARY KEY,
          wallet_id INT NOT NULL,
          amount DECIMAL(16, 8) NOT NULL,
          created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
          FOREIGN KEY (wallet_id) REFERENCES wallets(id)
      )")
  }

  pub fn create_mnemonic_table(&self) -> Result<(), Error> {
      self.execute_query(r"CREATE TABLE IF NOT EXISTS mnemonics (
          id INT AUTO_INCREMENT PRIMARY KEY,
          phrase TEXT NOT NULL,
          wallet_id INT NOT NULL,
          created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
          FOREIGN KEY (wallet_id) REFERENCES wallets(id)
      )")
  }

  pub fn get_connection(&self) -> Result<PooledConn, mysql::Error> {
      self.pool.get_conn()
  }

  pub fn close_connection(&self, conn: PooledConn) {
      drop(conn);
  }

   
}
