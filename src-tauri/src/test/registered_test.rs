use crate::{
    controllers::registered_controller::{
        create::create_questionnaire, delete::delete_questionnaire, read::read_registered,
        update::update_registered,
    },
    json_db::JsonDB,
    models::{
        questionnaire::{Question, Questionnaire},
        registered::RegisteredQuestionnaire,
        view_models::OperationResult,
    },
    QUESTIONNAIRE_COLLECTION, REGISTERED_COLLECTION, REGISTERED_DOCUMENT,
};

// Creates a questionnaire with one question through the controller call.
pub fn create_questionnaire_helper(name: &str) -> &str {
    // Arrange
    let questions = vec![Question::new_empty()];
    // Act
    let create_result = create_questionnaire(questions, &name);
    // Assert
    assert_eq!(create_result.result, OperationResult::Success);
    name
}

// Deletes a questionnaire using the controller call
pub fn cleanup_questionnaire(name: &str) {
    let delete_result = delete_questionnaire(name);
    assert_eq!(delete_result.result, OperationResult::Success);
}

#[test]
fn create_questionnaire_test() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let questions = vec![
        Question::new_empty(),
        Question::new_empty(),
        Question::new_empty(),
    ];
    let name: &str = "test_questionnaire";

    // Act
    let create_result = create_questionnaire(questions, name);

    // Assert
    assert_eq!(create_result.result, OperationResult::Success);

    //Cleanup
    let delete_result = delete_questionnaire(name);
    assert_eq!(delete_result.result, OperationResult::Success);
    Ok(())
}

#[test]
fn create_questionnaire_duplicate_name() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let name = create_questionnaire_helper("test_questionnaire");
    let questions = vec![Question::new_empty()];
    
    // Act
    let create_result = create_questionnaire(questions, name);

    // Assert
    assert_eq!(create_result.result, OperationResult::Fail);

    //Cleanup
    let delete_result = delete_questionnaire(name);
    assert_eq!(delete_result.result, OperationResult::Success);
    Ok(())
}

#[test]
fn read_directly_test() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let name = create_questionnaire_helper("test_questionnaire");

    // Act
    let registered: Vec<RegisteredQuestionnaire> =
        JsonDB::init("db")?.read(REGISTERED_COLLECTION, REGISTERED_DOCUMENT)?;

    // Assert
    assert_eq!(registered.len(), 1);
    assert_eq!(registered[0].name, "test_questionnaire");

    //Cleanup
    cleanup_questionnaire(name);
    Ok(())
}

#[test]
fn read_throough_controller() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let name = create_questionnaire_helper("test_questionnaire");

    // Act
    let result = read_registered();

    // Assert
    assert_eq!(result.result, OperationResult::Success);
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.data[0].name, "test_questionnaire");

    //Cleanup
    cleanup_questionnaire(name);
    Ok(())
}

#[test]
fn update_test() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    create_questionnaire_helper("test_questionnaire");
    let mut registered_questionnaires = read_registered();

    let new_name = "test_questionnaire_changed".to_string();
    registered_questionnaires.data[0].name = new_name.clone();

    // Act
    let update_result = update_registered(registered_questionnaires.data);
    let result = read_registered();

    // Assert
    assert_eq!(update_result.result, OperationResult::Success);
    assert_eq!(result.result, OperationResult::Success);
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.data[0].name, "test_questionnaire_changed");
    assert_eq!(result.data[0].document_name, "test_questionnaire");

    //Cleanup
    cleanup_questionnaire(&new_name);
    Ok(())
}

#[test]
fn delete_test() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let name = create_questionnaire_helper("test_questionnaire");

    // Act
    let delete_result = delete_questionnaire(name);
    let read_result = read_registered();

    // Assert
    assert_eq!(delete_result.result, OperationResult::Success);
    assert_eq!(read_result.result, OperationResult::Success);
    assert_eq!(read_result.data.len(), 0);
    Ok(())
}

#[test]
fn delete_test_two_questionnaires() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let name1 = create_questionnaire_helper("test_questionnaire");
    let name2 = create_questionnaire_helper("test_questionnaire_1");

    // Act
    let delete_result = delete_questionnaire(name1);
    let read_result = read_registered();

    // Assert
    assert_eq!(delete_result.result, OperationResult::Success);
    assert_eq!(read_result.result, OperationResult::Success);
    assert_eq!(read_result.data.len(), 1);
    assert_eq!(read_result.data[0].name, "test_questionnaire_1");

    // Cleanup
    cleanup_questionnaire(name2);
    Ok(())
}

#[test]
fn delete_after_name_change() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    create_questionnaire_helper("test_questionnaire");
    let new_name = "test_questionnaire_changed".to_string();

    let mut registered_questionnaires = read_registered();
    registered_questionnaires.data[0].name = new_name.clone();
    let update_result = update_registered(registered_questionnaires.data);
    let result = read_registered();

    // Assert
    assert_eq!(update_result.result, OperationResult::Success);
    assert_eq!(result.result, OperationResult::Success);
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.data[0].name, "test_questionnaire_changed");
    assert_eq!(result.data[0].document_name, "test_questionnaire");

    // Act II
    let delete_result = delete_questionnaire(&new_name);

    let list = JsonDB::init("db")?.list(QUESTIONNAIRE_COLLECTION)?;
    let read_result = read_registered();

    // Assert
    assert_eq!(delete_result.result, OperationResult::Success);
    assert_eq!(read_result.result, OperationResult::Success);
    assert_eq!(read_result.data.len(), 0);
    assert_eq!(list.len(), 0);
    Ok(())
}
