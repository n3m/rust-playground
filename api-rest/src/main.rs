#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod api;
mod models;

// use response::Response;

// use rocket::serde::json::Json;

// #[post("/", format = "json", data = "<carInput>")]
// fn createOne(carInput: CarInput) -> Json<Car> {
//     format!("Attempted to add car!")
//     Ok(Json<Car>({
//         id: '1',
//         name: 'Ferrari',
//         brand: 'Ferrari',
//         year: 2020
//     }))
// }

// #[put("/",  format = "json", data = "<car>")]
// fn updateOne(car: Car) -> Json<Car> {
//     format!("Attempted to update car!")
//     Ok(Json<Car>({
//         id: '1',
//         name: 'Ferrari',
//         brand: 'Ferrari',
//         year: 2020
//     }))
// }

// #[delete("/<id>")]
// fn delete_one(id: String) -> Json<Car> {
//     format!("Attempted to delete car with ID: '{id}'!")
//     Ok(Json<Car>({
//         id: '1',
//         name: 'Ferrari',
//         brand: 'Ferrari',
//         year: 2020
//     }))
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/car", routes![api::car::delivery::queries::query_one])
        .mount("/cars", routes![api::car::delivery::queries::query_all])
}
