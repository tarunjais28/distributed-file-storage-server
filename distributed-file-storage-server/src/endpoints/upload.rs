use super::*;

// API endpoint to upload a file in chunks
#[post("/upload")]
pub async fn upload_file(
    pool: web::Data<DbPool>, // Inject database connection pool
    mut payload: Multipart,  // The file is sent as a multipart form data payload
) -> Result<HttpResponse, CustomError> {
    let id = Uuid::new_v4(); // Generate a unique file ID for the uploaded file
    let (tx, rx) = channel(); // Create a channel for synchronizing chunk saving
    let mut filename = String::new(); // Variable to store the filename
    let mut total_chunks = 0; // Variable to track the total number of file chunks

    // Iterate through the multipart form data, processing each field
    while let Ok(Some(mut field)) = payload.try_next().await {
        // Get the content-disposition header, which contains the metadata, including filename
        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| CustomError::NullError)?;

        // Extract the filename from the content-disposition header
        if let Some(file_name) = content_disposition.get_filename() {
            filename = file_name.to_string(); // Set the filename
        }

        // Process the file data in chunks
        let pool = pool.clone(); // Clone the database pool to share between threads
        let tx = tx.clone(); // Clone the transmitter to share between threads
        let chunk_num = 0; // Initialize chunk number

        // Read the file chunk by chunk
        while let Some(chunk) = field.next().await {
            let chunk_data = chunk?.to_vec(); // Convert the chunk to a byte vector
            let pool = pool.clone(); // Clone the database pool again for thread usage
            let tx = tx.clone(); // Clone the transmitter for this chunk
            let mut chunk_num = chunk_num; // Assign chunk number to this chunk

            // Spawn a thread to insert the chunk into the database in parallel
            thread::spawn(move || {
                let mut conn = pool.get().expect("Error while connecting with database!");

                // Insert the chunk into the database, associating it with the file
                diesel::insert_into(chunks::table)
                    .values((
                        chunks::id.eq(Uuid::new_v4()),   // Unique ID for the chunk
                        chunks::file_id.eq(id),          // Associate the chunk with the file
                        chunks::chunk_num.eq(chunk_num), // The chunk number
                        chunks::data.eq(chunk_data),     // The actual data for the chunk
                    ))
                    .execute(&mut conn)
                    .expect("Error while inserting data into chunks table!");

                // Send a signal that the chunk has been saved successfully
                if let Err(err) = tx.send(()) {
                    eprintln!("{}", CustomError::SendError(err.to_string())); // Log error if sending fails
                }
            });

            chunk_num += 1; // Increment the chunk number for the next chunk
            total_chunks = chunk_num; // Update the total number of chunks processed
        }
    }

    // Wait for all chunks to be saved (one for each chunk)
    for _ in 0..total_chunks {
        rx.recv()?; // Receive a signal from each thread that the chunk is saved
    }

    // Clone the filename before moving it into the closure to keep it available
    let filename_clone = filename.clone();

    // Insert the file metadata (e.g., filename and total number of chunks) into the files table
    let pool = pool.clone(); // Clone the database pool for use in this thread
    thread::spawn(move || {
        let mut conn = pool.get().expect("Error while connecting with database!");

        // Insert the file metadata into the 'files' table
        diesel::insert_into(files::table)
            .values((
                files::id.eq(id),                    // File ID (same as the one used for the chunks)
                files::name.eq(filename_clone),      // The filename
                files::chunk_count.eq(total_chunks), // Total number of chunks
            ))
            .execute(&mut conn)
            .expect("Error while inserting data into files table!");
    })
    .join()
    .expect("Error while synchronization!"); // Ensure the file metadata insertion completes

    // Return a JSON response containing the file metadata (ID, filename, total chunks)
    Ok(HttpResponse::Ok().json(FileMetadata::new(
        id,
        filename,     // Return the original filename
        total_chunks, // Return the total number of chunks
    )))
}
