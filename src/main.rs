use std::fs::File;
use std::io::{self, Read, Error};

// Function to read a file and print its contents based on its encoding
fn read_file(path: &str) -> Result<(), Error> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Attempt to interpret the data as UTF-8 text; handle as binary if it fails
    match std::str::from_utf8(&contents) {
        Ok(text) => println!("File content as text: {}", text),
        Err(_) => println!("File content is not valid UTF-8, processed as binary data"),
    }

    Ok(())
}

fn main() {
    // Specific file paths for secret_file.txt and special_file.txt
    let file_paths = [
        r"C:\Users\cmill\OneDrive\Documents\OS\code\Project-3\test files\secret_file.txt",
        r"C:\Users\cmill\OneDrive\Documents\OS\code\Project-3\test files\special_file.txt"
    ];

    // Iterate over file paths and attempt to read each one
    for path in file_paths.iter() {
        println!("Reading file: {}", path);
        if let Err(e) = read_file(path) {
            eprintln!("Failed to read file: {}", e);
        }
    }
}


const REMOTE_SERVER_URL: &str = "http://127.0.0.1";

pub fn send_to_remote_server(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let message: &str = std::str::from_utf8(data)?;
    ureq::post(REMOTE_SERVER_URL).send_json(ureq::json!({
        "message": message,
        "github link": "https://github.com/NCGThompson/csci-485-project-3"
    }))?;
    Ok(())
}
