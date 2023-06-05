// use serde;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}
