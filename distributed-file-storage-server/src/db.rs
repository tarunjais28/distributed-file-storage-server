use super::*;

// Function to establish a connection pool to the database
pub fn establish_db_pool(con_str: &str) -> Result<DbPool, CustomError> {
    // Setup the connection manager for PostgreSQL using the provided connection string
    let conection_manager = ConnectionManager::<PgConnection>::new(con_str);

    // Create a connection pool using the connection manager
    // r2d2 is used to manage database connections efficiently, pooling them for reuse
    // The builder is configured to create the pool, and the pool is returned if successful
    Ok(r2d2::Pool::builder().build(conection_manager)?)
}
