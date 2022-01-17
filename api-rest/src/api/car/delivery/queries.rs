use crate::models::response::Response;
use rocket::serde::json::Json;
use uuid::Uuid;

#[get("/")]
pub fn query_all() -> Json<Response> {
  format!("Requested all cars");

  let uw_car_collection = crate::api::car::repository::CAR_COLLECTION.read().unwrap();
  if uw_car_collection.is_empty() {
    return Json(Response {
      status: 404,
      error: Some("No cars found".to_string()),
      value: None,
      values: None,
    });
  }

  Json(Response {
    status: 200,
    error: None,
    value: None,
    values: Some(uw_car_collection.values().cloned().collect()),
  })
}

#[get("/<id>")]
pub fn query_one(id: &str) -> Json<Response> {
  format!("Requested car with ID '{id}'!");

  let car_uuid = Uuid::parse_str(&id);
  if car_uuid.is_err() {
    return Json(Response {
      status: 400,
      error: Some("Invalid UUID".to_string()),
      value: None,
      values: None,
    });
  }

  let final_car_uuid = car_uuid.unwrap();

  let uw_car_collection = crate::api::car::repository::CAR_COLLECTION.read().unwrap();

  if uw_car_collection.contains_key(&final_car_uuid) {
    let uw_car = uw_car_collection.get(&final_car_uuid).unwrap();
    Json(Response {
      status: 200,
      value: Some(uw_car.clone()),
      error: None,
      values: None,
    })
  } else {
    Json(Response {
      status: 404,
      error: Some("Car not found".to_string()),
      value: None,
      values: None,
    })
  }
}
