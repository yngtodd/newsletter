use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    // Todo(Todd): don't leave these out to dry
    _name: String,
    _email: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
