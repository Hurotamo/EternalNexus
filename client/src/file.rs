pub fn read_file(file_path: &str) -> Result<Vec<u8>, String> {
    use std::fs::File;
    use std::io::Read;

    use std::fs::File;
    use std::io::Read;

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to open file: {}", e)),
    };

    let mut data = Vec::new(); 
    // Read the file contents into the data vector
    match file.read_to_end(&mut data) { 
        // Handle the result of reading the file
        Ok(_) => Ok(data),
        Err(e) => Err(format!("Failed to read file: {}", e)),
    }
}

pub fn read_sdata_file(file_path: &str) -> Result<Vec<u8>, String> {
    match read_file(file_path) {
        Ok(data) => {
            if is_encrypted(&data) {
                match decrypt_data(&data) {
                    Ok(decrypted_data) => Ok(decrypted_data),
                    Err(e) => Err(format!("Failed to decrypt data: {}", e)),
                }
            } else {
                Ok(data)
            }
        },
        Err(e) => Err(e),
    }
}

fn is_encrypted(data: &[u8]) -> bool { 
    // Logic to determine if the data is encrypted
    // Logic to determine if the data is encrypted
    // For example, check for a specific magic number
    data.starts_with(b"ENCRYPTED")
}

fn decrypt_data(data: &[u8]) -> Result<Vec<u8>, String> { 
    // Logic to decrypt the data
    // Logic to decrypt the data
    // For example, use a simple XOR cipher
    let key = b"secret_key";
    let mut decrypted_data = Vec::new();
    for (i, byte) in data.iter().enumerate() {
        decrypted_data.push(byte ^ key[i % key.len()]);
    }
    Ok(decrypted_data)
}
