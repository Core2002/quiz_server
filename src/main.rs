use rocket::{launch, routes};
pub mod routes;
mod service;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api",
        routes![routes::get_question_route],
    )
}
