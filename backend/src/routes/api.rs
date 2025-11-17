use actix_web::web;
use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/users")
                    .route("", web::get().to(handlers::user::get_all))
                    .route("/{id}", web::get().to(handlers::user::get_by_id))
                    .route("", web::post().to(handlers::user::create))
                    .route("/{id}", web::delete().to(handlers::user::delete))
            )
            .service(
                web::scope("/posts")
                    .route("", web::get().to(handlers::post::get_all))
                    .route("/user/{user_id}", web::get().to(handlers::post::get_by_user))
                    .route("", web::post().to(handlers::post::create))
                    .route("/{id}", web::delete().to(handlers::post::delete))
            )
    );
}