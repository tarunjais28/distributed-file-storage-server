use super::*;

pub fn create_file() -> Result<File, CustomError> {
    // Set the directory where you want to save the file
    let mut dir_path = PathBuf::from("/tmp");

    // Define base file name and extension
    let base_name = "download";
    let ext = "txt";
    let mut counter = 0;
    let mut file_name = format!("{}.{}", base_name, ext);

    // Loop to ensure unique file name if file already exists
    while Path::new(&dir_path.join(&file_name)).exists() {
        counter += 1;
        file_name = format!("{}({}).{}", base_name, counter, ext);
    }

    // Combine the directory and file name to create full path
    dir_path.push(&file_name);

    // Log the file path for debugging
    info!("Creating file at path: {:?}", dir_path);

    // Create or open the file at the specified path
    Ok(File::create(dir_path)?)
}
