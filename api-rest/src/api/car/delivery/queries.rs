use crate::models::response::Response;
use rocket::serde::json::Json;

#[get("/")]
pub fn query_all() -> Json<Response> {
  format!("Requested all cars");

  let operation_result =
    crate::api::car::repository::find(crate::api::car::filter::Filter { id: None });

  if operation_result.is_err() {
    return Json(Response {
      status: 404,
      error: Some(operation_result.err().unwrap()),
      value: None,
      values: None,
    });
  }

  Json(Response {
    status: 200,
    error: None,
    value: None,
    values: Some(operation_result.unwrap()),
  })
}

#[get("/<id>")]
pub fn query_one(id: &str) -> Json<Response> {
  format!("Requested car with ID '{id}'!");

  let operation_result = crate::api::car::repository::find_one(crate::api::car::filter::Filter {
    id: Some(id.to_string()),
  });
  if operation_result.is_err() {
    return Json(Response {
      status: 404,
      error: Some(operation_result.err().unwrap()),
      value: None,
      values: None,
    });
  }

  Json(Response {
    status: 200,
    error: None,
    value: Some(operation_result.unwrap()),
    values: None,
  })
}
