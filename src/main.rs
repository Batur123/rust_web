mod database;

#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use rocket::fs::{FileServer, relative};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build> // veya -> _
{
    rocket::build()
        .mount("/", routes![index])
        .mount("/public", FileServer::from(relative!("public/")))
}
