use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
  pub status: i32,
  pub error: Option<String>,
  pub value: Option<crate::models::car::Car>,
  pub values: Option<Vec<crate::models::car::Car>>,
}
