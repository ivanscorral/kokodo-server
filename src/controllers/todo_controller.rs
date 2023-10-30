use actix_web::HttpResponse;

pub async fn get_all_todos() -> HttpResponse {
    // TODO: Implement Logic
    HttpResponse::Ok().body("Here will be all todos")
}

pub async fn insert_todo(user_id: i32, title: String, description: Option<String>) -> HttpResponse {
    // TODO: Implement Logic
    HttpResponse::Ok().body("Here will be all todos")
}
