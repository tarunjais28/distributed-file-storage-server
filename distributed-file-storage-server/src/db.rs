use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_db_pool(con_str: &str) -> DbPool {
    // Setup Connection Manager
    let conection_manager = ConnectionManager::<PgConnection>::new(con_str);
    // Creating DB Pool
    r2d2::Pool::builder()
        .build(conection_manager)
        .expect("Failed to create pool.")
}
