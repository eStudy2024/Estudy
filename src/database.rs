use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;


pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub struct Database {
    pub pool: DBPool,
}


impl Database { 
    pub fn new() -> Self {
        let _ = dotenvy::dotenv();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Fail to build pool");
        Database { pool }
    }
}