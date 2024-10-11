use super::*;

// API to retrieve file metadata
#[get("/files")]
pub async fn get_files(pool: web::Data<DbPool>) -> Result<HttpResponse, CustomError> {
    let mut conn = pool.get()?;
    // Query DB to fetch file metadata (pseudo code)
    let files: Vec<FileMetadata> = files::table.load(&mut conn)?;

    Ok(HttpResponse::Ok().json(files))
}
