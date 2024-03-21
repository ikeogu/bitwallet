use mysql::{Pool, PooledConn, OptsBuilder, Error};


pub struct DBConnection {
    pool: Pool,
}

impl DBConnection {
    pub fn new() -> Result<Self, mysql::Error> {
        // Configure MySQL connection options
        let opts = OptsBuilder::new()
            .ip_or_hostname("localhost")
            .db_name("your_database_name")
            .user("your_username")
            .pass("your_password")
            .tcp_port(3306);

        // Create a connection pool
        let pool = Pool::new(opts)?;

        Ok(DBConnection { pool })
    }

    pub fn get_connection(&self) -> Result<PooledConn, mysql::Error> {
        self.pool.get_conn()
    }
}
