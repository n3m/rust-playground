use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct CarInput {
    pub name: String,
    pub brand: String,
    pub year: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Car {
    pub id: Uuid,
    pub name: String,
    pub brand: String,
    pub year: i32,
}

impl Car {
    pub fn new(name: String, brand: String, year: i32) -> Car {
        Car {
            id: Uuid::new_v4(),
            name,
            brand,
            year,
        }
    }
}
