use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredQuestionnaire {
    pub name: String,
    pub document_name: String,
    pub last_opened: String,
}

impl RegisteredQuestionnaire {
    pub fn new(name: String, document_name: String) -> RegisteredQuestionnaire {
        RegisteredQuestionnaire {
            name,
            document_name,
            last_opened: "0".to_string(),
        }
    }
}
