use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use crate::model::{RegisteredQuestionnaire, RegisteredQuestionnaires};

/// Will write a serializable element into the provided file path
///
/// # Errors
/// std::io::ErrorKind
/// This function will return an error if it cannot open or create the file.
pub fn write_to_json<T: Serialize>(
    serializable_element: T,
    file_path: String,
) -> std::io::Result<()> {
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

    let deserialized_json = serde_json::from_str(&json_string.as_str());

    match deserialized_json {
        Ok(successful) => successful,
        Err(err) => {
            eprintln!("Error deserializing JSON: {}", err);
            return None;
        }
    }
}

pub fn check_json_dir(directory_path: &str, questionnaires_file: &str) {
    if !fs::metadata(&directory_path).is_ok() {
        // Create the directory if it doesn't exist
        if let Err(err) = fs::create_dir_all(&directory_path) {
            eprintln!("Error creating directory: {}", err);
        } else {
            let new_registered_questionnaire_list = RegisteredQuestionnaires::new_empty();
            match write_to_json(
                new_registered_questionnaire_list,
                questionnaires_file.to_string(),
            ) {
                Ok(()) => println!("Empty file created successfully"),
                Err(err) => eprintln!("Attempting to create registered_questionnaires.json: {}", err),
            }
        }
    } else if !fs::metadata(&directory_path).is_ok() {
        let new_registered_questionnaire_list = RegisteredQuestionnaires::new_empty();
        match write_to_json(
            new_registered_questionnaire_list,
            questionnaires_file.to_string(),
        ) {
            Ok(()) => println!("Empty file created successfully"),
            Err(err) => eprintln!("Attempting to create registered_questionnaires.json: {}", err),
        }
    }
}
