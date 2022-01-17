use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

lazy_static! {
  pub static ref CAR_COLLECTION: RwLock<HashMap<Uuid, crate::models::car::Car>> = {
    let mut tmp_car_col = HashMap::new();

    for n in 1..=10 {
      let fcar =
        crate::models::car::Car::new("Corolla".to_string(), "Toyota".to_string(), 2020 + n);

      tmp_car_col.insert(fcar.id, fcar);
    }

    RwLock::new(tmp_car_col)
  };
}

pub fn find_one(
  filter: crate::api::car::filter::Filter,
) -> Result<crate::models::car::Car, String> {
  if filter.id == None {
    return Err("ID is required".to_string());
  }

  let car_uuid = Uuid::parse_str(filter.id.as_ref().unwrap());
  if car_uuid.is_err() {
    return Err("Invalid UUID".to_string());
  }

  let final_car_uuid = car_uuid.unwrap();

  let uw_car_collection = CAR_COLLECTION.read().unwrap();

  if uw_car_collection.contains_key(&final_car_uuid) {
    let uw_car = uw_car_collection.get(&final_car_uuid).unwrap();
    Ok(uw_car.clone())
  } else {
    Err("Car not found".to_string())
  }
}

pub fn find(
  _filter: crate::api::car::filter::Filter,
) -> Result<Vec<crate::models::car::Car>, String> {
  let uw_car_collection = CAR_COLLECTION.read().unwrap();

  if uw_car_collection.is_empty() {
    return Err("No cars found".to_string());
  }

  Ok(uw_car_collection.values().cloned().collect())
}
