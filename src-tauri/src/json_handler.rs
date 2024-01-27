use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::{fs::{self, OpenOptions}, io::Write};

/// Will write a serializable element into the provided file path
///
/// # Errors
/// std::io::ErrorKind
/// This function will return an error if it cannot open or create the file.
pub fn write_to_json<T: Serialize>(serializable_element: T, file_path: String) -> std::io::Result<()> {
    let json_string = serde_json::to_string_pretty(&serializable_element)?;
    
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)?;

    file.write_all(json_string.as_bytes())?;
    Ok(())
}

pub fn read_json<T: DeserializeOwned>(file_path: String) -> Option<T> {
    let json_string: String = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return None;
        }
    };

    let deserialized_json = serde_json::from_str(&json_string);
    
    match deserialized_json {
        Ok(successful) => successful,
        Err(err) => {
            eprintln!("Error deserializing JSON: {}", err);
            return None;
        }
    }
}
