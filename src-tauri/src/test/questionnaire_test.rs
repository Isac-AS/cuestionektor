use crate::{
    controllers::{
        questionnaire_controller::read::read_questionnaire,
        registered_controller::{create::create_questionnaire, delete::delete_questionnaire},
    },
    json_db::JsonDB,
    models::{
        questionnaire::{Question, Questionnaire},
        view_models::OperationResult,
    },
    QUESTIONNAIRE_COLLECTION,
};

// Creates a questionnaire with one question through the controller call.
fn create_questionnaire_helper(name: &str) -> &str {
    // Arrange
    let questions = vec![Question::new_empty()];
    // Act
    let create_result = create_questionnaire(questions, &name);
    // Assert
    assert_eq!(create_result.result, OperationResult::Success);
    name
}

// Deletes a questionnaire using the controller call
fn cleanup_questionnaire(name: &str) {
    let delete_result = delete_questionnaire(name);
    assert_eq!(delete_result.result, OperationResult::Success);
}

#[test]
fn create_questionnaire_test() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let questions = vec![Question::new_empty()];
    let name = "test_questionnaire";

    // Act
    let result = create_questionnaire(questions, name);

    // Assert
    assert_eq!(result.result, OperationResult::Success);

    // Cleanup
    cleanup_questionnaire(name);
    Ok(())
}

#[test]
fn read_questionnaire_directly_test() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let document = create_questionnaire_helper("test_questionnaire");
    
    // Act
    let questionnaire: Questionnaire =
        JsonDB::init("db")?.read(QUESTIONNAIRE_COLLECTION, document)?;
    
    // Assert
    assert_eq!(questionnaire.questions.len(), 1);

    // Cleanup
    cleanup_questionnaire(document);
    Ok(())
}

#[test]
fn read_questionnaire_through_controller() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let name = create_questionnaire_helper("test_questionnaire");

    // Act
    let result = read_questionnaire(name);

    // Assert
    assert_eq!(result.result, OperationResult::Success);
    assert_eq!(result.data.questions.len(), 3);
    
    // Cleanup
    cleanup_questionnaire(name);
    Ok(())
}
