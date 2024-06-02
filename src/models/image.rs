use ramhorns::Content;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize, Content)]
pub struct Image {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub width: i32,
    pub height: i32,
    pub thumbail: String,
}
