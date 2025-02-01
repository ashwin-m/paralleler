#[macro_use] extern crate rocket;

use rocket::response::status;
use rocket::form::Form;
use rocket::fs::TempFile;

#[derive(FromForm)]
struct Upload<'r> {
    file: TempFile<'r>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/submit", data = "<upload>")]
fn submit(upload: Form<Upload<'_>>) -> status::Accepted<&'static str> {
    status::Accepted("test")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, submit])
}
