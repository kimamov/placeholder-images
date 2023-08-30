use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Image {
    id: i32,
    name: String,
    url: String,
    width: i32,
    height: i32,
}
