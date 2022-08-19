use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct Alignment {
    alignment_id: i32,
    alignment_name: String,
}
