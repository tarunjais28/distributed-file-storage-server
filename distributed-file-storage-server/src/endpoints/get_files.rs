use super::*;

// API endpoint to retrieve metadata for all files
#[get("/files")]
pub async fn get_files(pool: web::Data<DbPool>) -> Result<HttpResponse, CustomError> {
    let mut conn = pool.get()?; // Get a database connection from the connection pool

    // Query the database to retrieve all file metadata
    // The `files::table` refers to the 'files' table in the database
    // This query loads all the records from the table into a vector of `FileMetadata` structs
    let files: Vec<FileMetadata> = files::table.load(&mut conn)?;

    // Return the file metadata as a JSON response with an HTTP 200 status
    Ok(HttpResponse::Ok().json(files))
}
