use std::error::Error;
use log::info;

use crate::model::{Answer, Question, PdfParsingFilters};

fn extract_pdf(file_path: &String) -> Result<String, Box<dyn Error>> {
    let bytes = std::fs::read(file_path)?;
    Ok(pdf_extract::extract_text_from_mem(&bytes).unwrap())
}

pub fn parse_pdf(file_path: String, filter_options: PdfParsingFilters) -> Vec<Question> {
    let text = extract_pdf(&file_path).expect("Could not read pdf.");

    let filtered_text = text.split("\n\n")
        .filter(|line| 
            filter_options.heading_re.is_match(&line) ||
            filter_options.possible_answer_re.is_match(&line) ||
            filter_options.answer_re.is_match(&line)
        );

    let mut questions: Vec<Question> = Vec::new();
    let mut placeholder_question = Question::new_empty();
    let prefixes = ["A) ", "B) ", "C) ", "D) "];
    filtered_text.for_each(|line| {

        // Removal and sanitation of line
        let lines: Vec<String> = line.split('\n').map(|v|String::from(v)).collect();
        let lines_without_last: Vec<String> = if lines.len() > 1 && line.contains("Página"){
            lines[..lines.len() - 1].to_vec()
        } else {
            lines.to_vec()
        };
        let line = lines_without_last.join(" ");
        
        // Extraction of heading and question number
        if filter_options.heading_re.is_match(line.as_str()) {
            placeholder_question.heading = line.to_string();
            let numeric_part: String = line.chars().take_while(|c| c.is_numeric()).collect();
            // Parse the numeric part into a u32
            if let Ok(number) = numeric_part.parse::<u32>() {
                placeholder_question.question_number = number;
            } else {
                info!("No valid u32 number found at the beginning of the string.");
            }
        }

        // Extraction of possible answer
        if filter_options.possible_answer_re.is_match(line.as_str()) {
            // Find the first matching prefix
            let stripped_line: Option<&str> = prefixes.iter().find_map(|&prefix| {
                line.strip_prefix(prefix).map(|rest| rest.trim())
            });
        
            placeholder_question.answers.push(
                Answer::new(
                    line.chars().next().unwrap().to_string(),
                    stripped_line.unwrap().to_string(),
                    false));
        }

        // Modification of answer and push into result
        if filter_options.answer_re.is_match(line.as_str()) {
            let correct_answer = line.chars().nth(line.len()-1).unwrap().to_string();
            placeholder_question.answers.iter_mut().for_each(|answer| {
                if answer.prefix == correct_answer {
                    answer.is_correct = true;
                }
            });
            
            questions.push(placeholder_question.clone());
            placeholder_question = Question::new_empty();
        }
    });
    questions
}
