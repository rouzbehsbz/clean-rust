use std::{io::Result, sync::Arc};

use actix_web::{web::Data, App, HttpServer};

use crate::{application::usecases::user::interface::IUserService, container::Container};

use super::routes::routes;

//TODO: better error result
pub async fn run() -> Result<()> {
    //TODO: better container implemention
    let container = Container::new();

    let user_service: Arc<dyn IUserService> = Arc::new(container.user_service);
    let user_service_data: Data<dyn IUserService> = Data::from(user_service);

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(user_service_data.clone())
            .configure(routes)
    });

    //TODO: use config isntead of hardcoded ip address
    server = server.bind("0.0.0.0:8080")?;

    server.run().await
}
