// We deny unsafe code because we want to be as secure as possible.
// If we really need it we can allow it on a case by case basis.
#[deny(clippy::pedantic, unsafe_code)]
#[forbid(clippy::undocumented_unsafe_blocks)]
#[macro_use]
extern crate rocket;

pub mod mojang;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
