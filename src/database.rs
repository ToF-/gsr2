use rusqlite::{params, Connection, Error};

#[derive(Debug)]
pub struct Database {
    connection: Connection,
}

impl Database {

    pub fn rusqlite_from_connection(connection_string: &str) -> Result<Self, Error> {
        match Connection::open(connection_string) {
            Ok(connection) => Ok(Database { connection }),
            Err(err) => Err(err),
        }
    }

    pub fn rusqlite_insert_picture(&self, picture: &Picture) -> Result<usize, Error> {
       self.connection.execute(
           "INSERT INTO Picture          \n\
           (Id,                          \n\
            Label)                       \n\
           VALUES (?1, ?2);",
           params![
           picture.file_path,
           picture.image_data])
    }

    pub fn rusqlite_retrieve_picture(&slef, file_path: &str) -> Result<Picture, Error> {
        self.connection.execute(
            "SELECT Id,                 \n\
             Label                      \n\
             FROM Picture               \n\
             WHERE Id = ?1;",
             params![ file_path ])
            .and_then(|mut rows| {
                if let Some(row) = rows.next()? {
                    Self::rusqlite_to_picture(row)
                } else {
                    Error
                }
            })
}
