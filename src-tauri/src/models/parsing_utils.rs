use tauri::regex::Regex;
// use tauri::regex;

pub struct ParsingFilters {
    pub heading_re: Regex,
    pub possible_answer_re: Regex,
    pub answer_re: Regex
}

impl ParsingFilters {
    pub fn new() -> ParsingFilters {
        ParsingFilters {
            heading_re: Regex::new(r"^\d{1,3}\.").unwrap(),
            possible_answer_re: Regex::new(r"^(A|B|C|D)\)").unwrap(),
            answer_re: Regex::new(r"^Respuesta Correcta").unwrap()
        }
    }
    // pub fn new_with_strings(heading_re: String, possible_answer_re: String, answer_re: String) -> ParsingFilters{
    //     ParsingFilters {
    //         heading_re: Regex::new(format!(r#"{}"#, regex::escape(&heading_re)).as_str()).unwrap(),
    //         possible_answer_re: Regex::new(format!(r#"{}"#, regex::escape(&possible_answer_re)).as_str()).unwrap(),
    //         answer_re: Regex::new(format!(r#"{}"#, regex::escape(&answer_re)).as_str()).unwrap()
    //     }
    // }
}