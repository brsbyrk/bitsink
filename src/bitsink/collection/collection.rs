use uuid::Uuid;

use super::Field;


pub struct Collection {
    id: Uuid,
    name: String,
    description: String,
    fields: Vec<Field>,
}






// CREATE TABLE collections (
//     id TEXT PRIMARY KEY,
//     project_id TEXT NOT NULL,
//     name TEXT NOT NULL,
//     FOREIGN KEY (project_id) REFERENCES projects (id)
// );

// CREATE TABLE fields (
//     id TEXT PRIMARY KEY,
//     collection_id TEXT NOT NULL,
//     name TEXT NOT NULL,
//     type TEXT NOT NULL,
//     FOREIGN KEY (collection_id) REFERENCES collections (id)
// );