#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod api;
mod models;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/car",
            routes![
                api::car::delivery::queries::query_one,
                api::car::delivery::mutations::create_one,
                api::car::delivery::mutations::update_one,
                api::car::delivery::mutations::delete_one,
            ],
        )
        .mount("/cars", routes![api::car::delivery::queries::query_all])
}
