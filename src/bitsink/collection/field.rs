use uuid::Uuid;


pub struct Field {
    id: Uuid,
    name: String,
    field_type: FieldType,
}

pub enum FieldType {
    Text,
    Number,
    Boolean,
    Date,
    Time,
    DateTime,
    Object,
    Array,
}