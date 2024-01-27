use serde_json;
use std::{fs::{self, File, OpenOptions}, io::{BufWriter, Write}};
use crate::model::Questionnaire;


pub fn write_questionnaire(questionnaire: Questionnaire) -> std::io::Result<()> {
    let json_string = serde_json::to_string_pretty(&questionnaire)?;
    
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&questionnaire.file_path)?;

    file.write_all(json_string.as_bytes())?;
    Ok(())
}

pub fn read_questionnaire(file_path: String) -> Option<Questionnaire> {
    let json_string = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return None;
        }
    };

    let questionnaire = serde_json::from_str(&json_string);
    
    match questionnaire {
        Ok(successful) => successful,
        Err(err) => {
            eprintln!("Error deserializing JSON: {}", err);
            return None;
        }
    }
}