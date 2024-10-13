use super::*;

/// Function to create a new file in a specified directory, ensuring the file name is unique.
/// Returns a `File` object or a `CustomError` if file creation fails.
pub fn create_file() -> Result<File, CustomError> {
    // Set the directory where the file will be saved (in this case, `/tmp`)
    let mut dir_path = PathBuf::from("/tmp");

    // Base name for the file (e.g., "download") and its extension (e.g., "txt")
    let base_name = "download";
    let ext = "txt";
    let mut counter = 0;

    // Start with a basic file name like "download.txt"
    let mut file_name = format!("{}.{}", base_name, ext);

    // Loop to ensure a unique file name in case the file already exists
    while Path::new(&dir_path.join(&file_name)).exists() {
        // Increment the counter and update the file name to include the counter value
        // e.g., "download(1).txt", "download(2).txt", etc.
        counter += 1;
        file_name = format!("{}({}).{}", base_name, counter, ext);
    }

    // Append the unique file name to the directory path
    dir_path.push(&file_name);

    // Log the file path for debugging purposes (e.g., to know where the file was created)
    info!("Creating file at path: {:?}", dir_path);

    // Create or open the file at the specified path
    // If successful, return the `File` object; otherwise, return an I/O error wrapped in a `CustomError`
    Ok(File::create(dir_path)?)
}
