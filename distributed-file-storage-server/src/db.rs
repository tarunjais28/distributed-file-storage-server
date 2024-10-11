use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

use crate::CustomError;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_db_pool(con_str: &str) -> Result<DbPool, CustomError> {
    // Setup Connection Manager
    let conection_manager = ConnectionManager::<PgConnection>::new(con_str);
    // Creating DB Pool
    Ok(r2d2::Pool::builder().build(conection_manager)?)
}
