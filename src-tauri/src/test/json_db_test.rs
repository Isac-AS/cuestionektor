use crate::{json_db::JsonDB, QUESTIONNAIRE_COLLECTION, REGISTERED_COLLECTION};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn init() -> Result<(), Box<dyn std::error::Error>> {
    let _db = JsonDB::init("db")?;
    Ok(())
}

#[test]
fn create_collection() -> Result<(), Box<dyn std::error::Error>> {
    // Act
    JsonDB::init("db")?.create_collection("test")?;
    // Cleanup
    JsonDB::init("db")?.delete_collection("test")?;
    Ok(())
}

#[test]
fn write() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange 
    JsonDB::init("db")?.create_collection("test")?;

    // Act
    JsonDB::init("db")?.write("test", "test", Point { x: 3, y: 5 })?;

    // Cleanup
    JsonDB::init("db")?.delete("test", "test")?;
    JsonDB::init("db")?.delete_collection("test")?;
    Ok(())
}

#[test]
fn read() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange 
    JsonDB::init("db")?.create_collection("test")?;
    JsonDB::init("db")?.write("test", "test", Point { x: 3, y: 5 })?;

    // Act
    let test: Point = JsonDB::init("db")?.read("test", "test")?;

    // Assert
    assert_eq!(test.x, 3);
    assert_eq!(test.y, 5);

    // Cleanup
    JsonDB::init("db")?.delete("test", "test")?;
    JsonDB::init("db")?.delete_collection("test")?;
    Ok(())
}

#[test]
fn delete() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange 
    JsonDB::init("db")?.create_collection("test")?;
    JsonDB::init("db")?.write("test", "test", Point { x: 3, y: 5 })?;

    // Act
    JsonDB::init("db")?.delete("test", "test")?;

    // Cleanup
    JsonDB::init("db")?.delete_collection("test")?;
    Ok(())
}

#[test]
fn list() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange 
    JsonDB::init("db")?.create_collection("books")?;
    JsonDB::init("db")?.write("books", "cavalry", "data")?;
    JsonDB::init("db")?.write("books", "fantasy", "data")?;
    let collections = vec!["cavalry".to_string(), "fantasy".to_string()];

    // Act 
    let list = JsonDB::init("db")?.list("books")?;

    // Assert
    assert!(list.contains(&collections[0]));
    assert!(list.contains(&collections[1]));

    // Cleanup
    JsonDB::init("db")?.delete("books", "cavalry")?;
    JsonDB::init("db")?.delete("books", "fantasy")?;
    JsonDB::init("db")?.delete_collection("books")?;
    Ok(())
}

#[test]
fn init_collections() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let db = JsonDB::init("db")?;

    // Act
    db.init_collections()?;

    // Assert
    let registered = JsonDB::init("db")?.list(REGISTERED_COLLECTION)?;
    let questionnaires = JsonDB::init("db")?.list(QUESTIONNAIRE_COLLECTION)?;
    assert_eq!(registered.len(), 0);
    assert_eq!(questionnaires.len(), 0);
    Ok(())
}
