use actix_web::web;
use crate::controllers::todo_controller;

pub fn init(cfg: &mut  web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .route("", web::get().to(todo_controller::get_all_todos))
    );
}
