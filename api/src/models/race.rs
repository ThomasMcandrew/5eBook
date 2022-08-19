use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Race {
    race_id: i32,
    race_name: String,
}
