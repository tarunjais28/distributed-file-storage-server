use super::*;

// File metadata
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct FileMetadata {
    id: Uuid,
    name: String,
    chunk_count: i32,
}

impl FileMetadata {
    pub fn new(id: Uuid, name: String, chunk_count: i32) -> Self {
        Self {
            id,
            name,
            chunk_count,
        }
    }
}
