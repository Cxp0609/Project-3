use std::fs;
use std::io::{self, Read};
use aes::{
    cipher::{block_padding::Pkcs7, BlockDecryptMut as _, KeyIvInit as _},
    Aes192,
};

use cbc;

/// Custom error type to represent various error conditions.
#[derive(Debug)]
pub enum CustomError {
    FileNotFound(String),
    IoError(std::io::Error),
    SendError(String),
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomError::FileNotFound(file) => write!(f, "File not found: {}", file),
            CustomError::IoError(err) => write!(f, "IO Error: {}", err),
            CustomError::SendError(msg) => write!(f, "Send Error: {}", msg),
        }
    }
}

impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CustomError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        CustomError::IoError(err)
    }
}

/// Main function that orchestrates the file decryption and remote server communication.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let special_file_path = find_file("/", "special_file.txt").ok_or_else(|| CustomError::FileNotFound("special_file.txt".to_string()))?;
    let secret_file_path = find_file("/", "secret_file.txt").ok_or_else(|| CustomError::FileNotFound("secret_file.txt".to_string()))?;

    let special_file_contents = fs::read_to_string(&special_file_path)?;
    println!("Contents of special_file.txt:\n{}", special_file_contents);

    let secret_file_contents = fs::read_to_string(&secret_file_path)?;
    println!("Contents of secret_file.txt:\n{}", secret_file_contents);

    let secret_file_contents = fs::read(&secret_file_path)?;
    let secret_file_bytes = secret_file_contents.as_slice();

    let special_file_contents = fs::read(&special_file_path)?;
    let special_file_bytes = special_file_contents.as_slice();

    // Decrypt secret file contents
    let key = secret_file_bytes; // Example key, replace with your actual key
    let decrypted_data = decrypt_file(special_file_bytes, key)?;
    println!("Decrypted secret file contents:\n{}", String::from_utf8_lossy(&decrypted_data));

    // Send secret file contents to remote server
    match send_to_remote_server(&decrypted_data) {
        Ok(_) => println!("Data sent to remote server successfully"),
        Err(err) => eprintln!("Error sending data to remote server: {}", err),
    }

    Ok(())
}

/// Function to find a file by name starting from a root directory.
pub fn find_file(root_dir: &str, target_file: &str) -> Option<std::path::PathBuf> {
    let mut stack = vec![std::path::PathBuf::from(root_dir)];

    while let Some(dir) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        stack.push(path);
                    } else if path.file_name() == Some(std::ffi::OsStr::new(target_file)) {
                        return Some(path);
                    }
                }
            }
        }
    }
    None
}

pub const REMOTE_SERVER_URL: &str = "http://127.0.0.1";

/// Function to send data to a remote server.
pub fn send_to_remote_server(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let message: &str = std::str::from_utf8(data)?;
    ureq::post(REMOTE_SERVER_URL).send_json(ureq::json!({
        "message": message,
        "github link": "https://github.com/Cxp0609/Project-3"
    }))?;
    Ok(())
}

/// Decrypts a file containing AES-192 CBC encrypted data.
///
/// # Errors
///
/// This function will return an error if decryption fails.
pub fn decrypt_file(ciphertext: &[u8], key: &[u8]) -> io::Result<Vec<u8>> {
    let iv = [0u8; 16]; // initialization vector

    let cipher = cbc::Decryptor::<Aes192>::new_from_slices(key, &iv)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    cipher
        .decrypt_padded_vec_mut::<Pkcs7>(&ciphertext)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test case for decrypt_file function.
    #[test]
    pub fn test_decrypt_file() {
        // Example ciphertext and key (replace with your actual data)
        let ciphertext = b"t7w46xqRG9a/czWe9+J0jn9yClEuVDbhE9kJJYF2l8g=";
        let key = b"6a69a5d17620fc4d1536f7f2476487e9";

        // Decrypt the ciphertext
        let result = decrypt_file(ciphertext, key);

        // Assert that decryption was successful
        assert!(result.is_ok());

        // Optionally, add more assertions here to verify the decrypted data
    }

    // Test case for find_file function.
    #[test]
    pub fn test_find_file() {
        // Create a temporary directory and files for testing
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let special_file_path = temp_dir.path().join("special_file.txt");
        let secret_file_path = temp_dir.path().join("secret_file.txt");

        // Write some content to the files
        fs::write(&special_file_path, "special file content").expect("Failed to write special file");
        fs::write(&secret_file_path, "secret file content").expect("Failed to write secret file");

        // Test finding the files
        assert_eq!(find_file(temp_dir.path().to_str().unwrap(), "special_file.txt"), Some(special_file_path));
        assert_eq!(find_file(temp_dir.path().to_str().unwrap(), "secret_file.txt"), Some(secret_file_path));
    }

    // Test case for send_to_remote_server function.
    #[test]
    pub fn test_send_to_remote_server() {
        // Example data to send
        let data = b"example data";

        // Test sending data to remote server
        assert!(send_to_remote_server(data).is_ok());
    }
}
