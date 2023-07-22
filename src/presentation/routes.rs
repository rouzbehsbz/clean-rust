use actix_web::web::{self, ServiceConfig};

use super::controllers::users::handler::register;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::scope("/api").service(
            web::scope("/v1").service(web::scope("/users").route("", web::post().to(register))),
        ));
}
