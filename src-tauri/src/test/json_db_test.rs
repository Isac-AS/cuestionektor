use crate::json_db::JsonDB;
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
    JsonDB::init("db")?.create_collection("test")?;
    Ok(())
}

#[test]
fn write() -> Result<(), Box<dyn std::error::Error>> {
    JsonDB::init("db")?.write("test", "test", Point { x: 3, y: 5 })?;
    Ok(())
}

#[test]
fn read() -> Result<(), Box<dyn std::error::Error>> {
    let test: Point = JsonDB::init("db")?.read("test", "test")?;
    assert_eq!(test.x, 3);
    assert_eq!(test.x, 5);
    Ok(())
}

#[test]
fn delete() -> Result<(), Box<dyn std::error::Error>> {
    JsonDB::init("db")?.delete("books", "ell")?;
    Ok(())
}

#[test]
fn list() -> Result<(), Box<dyn std::error::Error>> {
    let list = JsonDB::init("db")?.list("books")?;
    let list2 = vec!["lol", "test"];
    assert_eq!(list.get(0).unwrap(), list2.get(0).unwrap());
    assert_eq!(list.get(1).unwrap(), list2.get(1).unwrap());
    Ok(())
}
