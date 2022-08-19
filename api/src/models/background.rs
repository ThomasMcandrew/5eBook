use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Background {
    background_id: i32,
    background_name: String,
}
