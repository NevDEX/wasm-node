#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

mod headler;

#[launch]
fn rocket() -> _ {
   rocket::build().mount("/json",routes![headler::body_data_json])
}
