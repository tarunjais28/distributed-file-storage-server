use super::*;

// Structure to represent file metadata, including ID, name, and chunk count
#[derive(Debug, Queryable, Serialize, Deserialize)] 
pub struct FileMetadata {
    id: Uuid,            // Unique identifier for the file
    name: String,        // Name of the file
    chunk_count: i32,    // Number of chunks the file is split into
}

impl FileMetadata {
    // Constructor to create a new instance of FileMetadata
    pub fn new(id: Uuid, name: String, chunk_count: i32) -> Self {
        // Return a new instance of FileMetadata with the provided values
        Self {
            id,           // File ID
            name,         // File name
            chunk_count,  // Number of chunks
        }
    }
}
