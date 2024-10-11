use super::*;

#[post("/upload")]
pub async fn upload_file(
    pool: web::Data<DbPool>,
    mut payload: Multipart,
) -> Result<HttpResponse, CustomError> {
    let id = Uuid::new_v4(); // Generate unique file ID
    let (tx, rx) = channel();
    let mut filename = String::new();
    let mut total_chunks = 0;

    // Iterate through the multipart form data to process the file
    while let Ok(Some(mut field)) = payload.try_next().await {
        // Get the content-disposition header which contains the filename
        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| CustomError::NullError)?;

        // Extract the filename from the content-disposition metadata
        if let Some(file_name) = content_disposition.get_filename() {
            filename = file_name.to_string();
        }

        // Process the file data in chunks
        let pool = pool.clone();
        let tx = tx.clone();
        let chunk_num = 0;

        while let Some(chunk) = field.next().await {
            let chunk_data = chunk?.to_vec();
            let pool = pool.clone();
            let tx = tx.clone();
            let mut chunk_num = chunk_num;

            // Spawn a thread for each chunk to save it in the database in parallel
            thread::spawn(move || {
                let mut conn = pool.get().expect("Error while connecting with database!");

                // Insert each chunk into the chunks table
                diesel::insert_into(chunks::table)
                    .values((
                        chunks::id.eq(Uuid::new_v4()), // Unique chunk ID
                        chunks::file_id.eq(id),        // Associate the chunk with the file
                        chunks::chunk_num.eq(chunk_num),
                        chunks::data.eq(chunk_data),
                    ))
                    .execute(&mut conn)
                    .expect("Error while inserting data to chunks table!!");

                // Send a message after the chunk is inserted
                if let Err(err) = tx.send(()) {
                    eprintln!("{}", CustomError::SendError(err.to_string()));
                }
            });

            chunk_num += 1;
            total_chunks = chunk_num; // Update the total number of chunks
        }
    }

    // Wait for all chunks to be saved
    for _ in 0..total_chunks {
        rx.recv()?;
    }

    // Clone filename before moving it into the closure
    let filename_clone = filename.clone();

    // Insert file metadata into the files table
    let pool = pool.clone();
    thread::spawn(move || {
        let mut conn = pool.get().expect("Error while connecting with database!");
        diesel::insert_into(files::table)
            .values((
                files::id.eq(id),                    // The same file ID used for the chunks
                files::name.eq(filename_clone),      // The filename
                files::chunk_count.eq(total_chunks), // Number of chunks saved
            ))
            .execute(&mut conn)
            .expect("Error while inserting data to files table!!");
    })
    .join()
    .expect("Error while synchronization!!"); // Ensure the metadata insertion completes

    // Return response with file metadata
    Ok(HttpResponse::Ok().json(FileMetadata::new(
        id,
        filename,     // Return the actual filename (not moved)
        total_chunks, // Return the number of chunks
    )))
}
