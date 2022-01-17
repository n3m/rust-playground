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
