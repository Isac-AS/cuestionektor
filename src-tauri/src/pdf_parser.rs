use std::error::Error;
use crate::model::{Answer, Question};
use tauri::regex::Regex;


pub fn extract_pdf(file_path: String) -> Result<String, Box<dyn Error>> {
    let bytes = std::fs::read(file_path)?;
    Ok(pdf_extract::extract_text_from_mem(&bytes).unwrap())
}

pub fn parse_pdf(file_path: String) -> Vec<Question> {
    let text = extract_pdf(file_path).expect("Could not read pdf.");
    let heading_re = Regex::new("");
    text
        .split('\n')
        .into_iter()
        .filter(
            |line| {
            match *line {

            }
        });
    
    vec![Question::new(
        0,
        file_path, 
        vec![Answer::new(file_path, file_path, false)],
        file_path,
        file_path
    )]
}
