use rocket::serde::{Deserialize, Serialize};

use crate::models::car::Car;

#[derive(Serialize, Deserialize)]
pub struct Response {
  pub status: i32,
  pub error: Option<String>,
  pub value: Option<Car>,
  pub values: Option<Vec<Car>>,
}
