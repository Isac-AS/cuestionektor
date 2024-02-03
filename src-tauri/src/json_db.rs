use serde::{Deserialize, Serialize};
use std::{ fs, path::Path};

/// Database struct (does not use `::new` instead uses `::init`)
///
/// # Examples
/// ```
/// let db = JsonDB::init("your db name")?;
/// ```
///
/// ### Note:
/// Does not handle errors in version "0.1.0" just passes on to user
///
pub struct JsonDB {
    path: String,
}

impl JsonDB {
    /// Init function (takes the place of `::new`)
    ///
    /// # Examples
    /// ```
    /// let db = JsonDB::init("your db name")?;
    /// ```
    ///
    pub fn init(path: &str) -> Result<JsonDB,Box<dyn std::error::Error>> {
        let db_path = format!("./.JsonDB/{}/", path);
        if !Path::new(&*format!("./.JsonDB/{}/", path)).is_dir() {
            fs::create_dir_all(format!("./.JsonDB/{}/", path))?;
        }
        Ok(JsonDB { path: db_path })
    }

    pub fn init_collections(self) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(&*format!("{}/questionnaires/", &self.path)).is_dir() {
            fs::create_dir_all(format!("{}/questionnaires/", &self.path))?;
        }
        if !Path::new(&*format!("{}/registered/", &self.path)).is_dir() {
            fs::create_dir_all(format!("{}/registered/", &self.path))?;
        }
        Ok(())
    }

    /// Creates a new collection either in your database or in another collection
    ///
    /// # Examples
    /// ```
    /// db.create_collection("your collection path")?;
    /// ```
    ///
    pub fn create_collection<S>(
        self,
        collection_path: S,
    ) -> Result<JsonDB, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        fs::create_dir(format!("{}/{}", self.path, collection_path.into()))?;
        Ok(self)
    }

    /// Writes (or rewrites) data to document in collection
    ///
    /// # Examples
    /// ```
    /// db.write("your collection path", "your document", "struct that derives serde::serialize")?;
    /// ```
    ///
    pub fn write<J, S>(
        self,
        collection_path: S,
        document: S,
        data: J,
    ) -> Result<JsonDB, Box<dyn std::error::Error>>
    where
        J: Serialize,
        S: Into<String>,
    {
        let serialized = serde_json::to_string(&data)?;
        fs::write(
            format!(
                "{}/{}/{}.data.json",
                self.path,
                collection_path.into(),
                document.into()
            ),
            serialized,
        )?;
        Ok(self)
    }

    /// Reads data from document in collection
    ///
    /// ```
    /// let data: impl serde::Deserialize = db.read("your collection path", "your document")?;
    /// ```
    ///
    pub fn read<D, S>(
        self,
        collection_path: S,
        document: S,
    ) -> Result<D, Box<dyn std::error::Error>>
    where
        for<'a> D: Deserialize<'a>,
        S: Into<String>,
    {
        let data = serde_json::from_str::<D>(
            fs::read_to_string(format!(
                "{}/{}/{}.data.json",
                self.path,
                collection_path.into(),
                document.into()
            ))?
            .as_str(),
        )?;
        Ok(data)
    }

    /// Deletes document in collection
    ///
    /// # Examples
    /// ```
    /// db.delete("your collection path", "your document")?;
    /// ```
    ///
    pub fn delete<S>(
        self,
        collection_path: S,
        document: S,
    ) -> Result<JsonDB, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        fs::remove_file(format!(
            "{}/{}/{}.data.json",
            self.path,
            collection_path.into(),
            document.into()
        ))?;
        Ok(self)
    }

    /// List all document in a collection
    ///
    /// # Examples
    /// ```
    /// let list: Vec<String> = db.list("your collection path")?;
    /// ```
    ///
    pub fn list<S>(self, collection_path: S) -> Result<Vec<String>, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        let mut list: Vec<String> = Vec::new();
        for file in fs::read_dir(format!("{}/{}/", self.path, collection_path.into()))? {
            let mut value = file?.file_name().into_string().unwrap();
            let len = value.chars().count() - 10;
            let string = String::from(value.drain(0..len).as_str());

            list.push(string);
        }
        Ok(list)
    }
}