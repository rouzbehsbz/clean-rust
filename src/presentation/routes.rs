use actix_web::web::{self, ServiceConfig};

use super::controllers::users::handler::{login, register, update_profile};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/v1").service(
                web::scope("/users")
                    .route("", web::post().to(register))
                    .route("/session", web::post().to(login))
                    .route("/test", web::put().to(update_profile))
            ),
        ),
    );
}
