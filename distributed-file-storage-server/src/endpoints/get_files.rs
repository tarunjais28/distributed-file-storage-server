use super::*;

// API to retrieve file metadata
#[get("/files")]
pub async fn get_files(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().unwrap();
    // Query DB to fetch file metadata (pseudo code)
    let files: Vec<FileMetadata> = files::table.load(&mut conn).unwrap();

    HttpResponse::Ok().json(files)
}
