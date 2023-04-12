use rocket::{get, http::CookieJar};

use crate::service::next;

#[get("/next_question")]
pub async fn get_question_route(cookies: &CookieJar<'_>) -> String {
    next().to_json()
}
