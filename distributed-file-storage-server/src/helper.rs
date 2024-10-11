use super::*;

pub fn create_file() -> Result<File, CustomError> {
    // Create or open the file to write the downloaded data
    let mut counter = 0;
    let base_name = "download";
    let ext = "txt";
    let mut file_name = format!("{}.{}", base_name, ext);

    // Loop until we find a file name that doesn't exist
    while Path::new(&file_name).exists() {
        counter += 1;
        file_name = format!("{}({}).{}", base_name, counter, ext);
    }
    Ok(File::create(file_name)?)
}
