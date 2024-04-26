use std::fs;
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Find the paths of special_file.txt and secret_file.txt
    let special_file_path = find_file("/", "special_file.txt").ok_or("Special file not found")?;
    let secret_file_path = find_file("/", "secret_file.txt").ok_or("Secret file not found")?;

    // Read the contents of the files and print them
    let special_file_contents = fs::read_to_string(&special_file_path)?;
    println!("Contents of special_file.txt: \n{}", special_file_contents);

    let secret_file_contents = fs::read_to_string(&secret_file_path)?;
    println!("Contents of secret_file.txt: \n{}", secret_file_contents);

    Ok(())
}

fn find_file(root_dir: &str, target_file: &str) -> Option<String> {
    let mut stack = vec![Path::new(root_dir).to_path_buf()];

    while let Some(dir) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        stack.push(path);
                    } else if path.file_name().unwrap_or_default() == target_file {
                        return Some(path.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }
    None
}
