pub const COSTUMES_DEFINITIONS_FILE: &str = "data/Item/DualLayerClothes.SData";
pub const COSTUMES_NOT_FOUND_MESSAGE: &str = "DualLayerClothes.SData was not found.";

pub fn load_costume_definitions() -> Result<(), String> {
    // Load the definition file.
    match read_sdata_file(COSTUMES_DEFINITIONS_FILE) {
        Ok(data) => {
            // Store the data in a safe and accessible structure
            let costume_definitions = unsafe { std::slice::from_raw_parts(data as *const u8, 1024) };
            // Process the data
            Ok(())
        },
        Err(_) => {
            Err(COSTUMES_NOT_FOUND_MESSAGE.to_string())
        }
    }
}

// Function to read the SData file
fn read_sdata_file(file_path: &str) -> Result<*mut u8, String> {
    use std::fs::File;
    use std::io::Read;

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return Err(format!("Failed to open {}", file_path)),
    };

    let mut data = Vec::new();
    match file.read_to_end(&mut data) {
        Ok(_) => Ok(data.as_mut_ptr()),
        Err(_) => Err(format!("Failed to read {}", file_path)),
    }
}
