use uuid::Uuid;

use super::Field;


pub struct Collection {
    id: Uuid,
    name: String,
    description: String,
    fields: Vec<Field>,
}