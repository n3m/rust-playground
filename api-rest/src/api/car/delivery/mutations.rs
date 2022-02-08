use crate::models::{
  car::{Car, CarInput},
  response::Response,
};
use rocket::serde::json::Json;

#[post("/", format = "json", data = "<car_input>")]
pub fn create_one(car_input: Json<CarInput>) -> Json<Response> {
  let operation_result = crate::api::car::repository::insert_one(car_input.clone());

  if operation_result.is_err() {
    return Json(Response {
      status: 404,
      error: Some(operation_result.err().unwrap()),
      value: None,
      values: None,
    });
  }

  return Json(Response {
    status: 200,
    error: None,
    value: Some(operation_result.unwrap()),
    values: None,
  });
}

#[put("/", format = "json", data = "<car>")]
pub fn update_one(car: Json<Car>) -> Json<Response> {
  let uw_car = car.clone();
  if uw_car.id.is_nil() {
    return Json(Response {
      status: 404,
      error: Some("ID is required".to_string()),
      value: None,
      values: None,
    });
  }
  let operation_result = crate::api::car::repository::update_one(uw_car);
  if operation_result.is_err() {
    return Json(Response {
      status: 404,
      error: Some(operation_result.err().unwrap()),
      value: None,
      values: None,
    });
  }
  return Json(Response {
    status: 200,
    error: None,
    value: Some(operation_result.unwrap()),
    values: None,
  });
}

#[delete("/<id>")]
pub fn delete_one(id: String) -> Json<Response> {
  let operation_result =
    crate::api::car::repository::delete_one(crate::api::car::filter::Filter { id: Some(id) });

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
    values: None,
  })
}
