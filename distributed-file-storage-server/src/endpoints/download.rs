use super::*;

// API endpoint to download a file by its ID
#[get("/download/{file_id}")]
pub async fn download_file(
    pool: web::Data<DbPool>,  // Injecting the database connection pool
    file_id: web::Path<Uuid>, // The file ID is passed in the request path as a UUID
) -> Result<HttpResponse, CustomError> {
    let mut conn = pool.get()?; // Get a database connection from the pool
    let file_id = file_id.into_inner(); // Extract the file ID from the web path

    // Query the database to retrieve the file chunks that match the file_id
    let fetched_chunks: Vec<(i32, Vec<u8>)> = chunks::table
        .filter(chunks::file_id.eq(file_id)) // Filter chunks by file ID
        .select((chunks::chunk_num, chunks::data)) // Select chunk number and chunk data
        .order(chunks::chunk_num.asc()) // Order chunks by chunk number in ascending order
        .load::<(i32, Vec<u8>)>(&mut conn)?; // Execute the query and load results into a Vec

    // If no chunks are found for the given file ID, return a 404 Not Found response
    if fetched_chunks.is_empty() {
        return Ok(HttpResponse::NotFound().body("File not found"));
    }

    // Create a channel to send chunks between threads for parallel processing
    let (tx, rx) = channel();

    // Spawn a thread for each chunk to handle it in parallel
    for (chunk_num, chunk_data) in fetched_chunks.clone() {
        let tx = tx.clone(); // Clone the transmitter so each thread can send back data
        thread::spawn(move || {
            // Send the chunk data back through the channel, handle errors
            if let Err(err) = tx.send((chunk_num, chunk_data)) {
                eprintln!("{}", CustomError::SendError(err.to_string())); // Log error if sending fails
            }
        });
    }

    // Initialize a vector to hold the sorted chunks
    let mut sorted_chunks = vec![None; fetched_chunks.len()]; // Preallocate space for each chunk

    // Receive chunks from the threads and place them in the correct order
    for _ in 0..fetched_chunks.len() {
        let (chunk_num, chunk_data) = rx.recv()?; // Receive chunk number and data from the channel
        sorted_chunks[chunk_num as usize] = Some(chunk_data); // Store the chunk data at the correct position
    }

    // Create a new file to store the reassembled chunks
    let mut file = create_file()?;

    // Write the chunks to the file in order
    for chunk in sorted_chunks.into_iter().flatten() {
        // Flatten the optional chunks and iterate over them
        file.write_all(&chunk)?; // Write each chunk to the file
    }

    // Return a success response once the file has been fully downloaded
    Ok(HttpResponse::Ok().body("File downloaded successfully"))
}
