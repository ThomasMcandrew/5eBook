use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Class {
    class_id: i32,
    class_name: String,
}
