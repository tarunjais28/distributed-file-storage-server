use super::*;

// API to download file by ID
#[get("/download/{file_id}")]
pub async fn download_file(
    pool: web::Data<DbPool>,
    file_id: web::Path<Uuid>,
) -> Result<HttpResponse, CustomError> {
    let mut conn = pool.get()?;
    let file_id = file_id.into_inner();

    // Fetch file chunks from DB that match the file_id
    let fetched_chunks: Vec<(i32, Vec<u8>)> = chunks::table
        .filter(chunks::file_id.eq(file_id))
        .select((chunks::chunk_num, chunks::data))
        .order(chunks::chunk_num.asc())
        .load::<(i32, Vec<u8>)>(&mut conn)?;

    if fetched_chunks.is_empty() {
        return Ok(HttpResponse::NotFound().body("File not found"));
    }

    // Create a channel to collect the chunks after they are processed in parallel
    let (tx, rx) = channel();

    // Spawn threads to handle each chunk in parallel
    for (chunk_num, chunk_data) in fetched_chunks.clone() {
        let tx = tx.clone();
        thread::spawn(move || {
            // Send the chunk data back through the channel
            if let Err(err) = tx.send((chunk_num, chunk_data)) {
                eprintln!("{}", CustomError::SendError(err.to_string()));
            }
        });
    }

    // Collect and write the chunks in the correct order
    let mut sorted_chunks = vec![None; fetched_chunks.len()];
    for _ in 0..fetched_chunks.len() {
        let (chunk_num, chunk_data) = rx.recv()?;
        sorted_chunks[chunk_num as usize] = Some(chunk_data);
    }

    let mut file = create_file()?;
    for chunk in sorted_chunks.into_iter().flatten() {
        file.write_all(&chunk)?;
    }

    Ok(HttpResponse::Ok().body("File downloaded successfully"))
}
