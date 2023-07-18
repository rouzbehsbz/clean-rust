use std::io::Result;

use actix_web::{App, HttpServer};

use super::routes::routes;

//TODO: better error result
pub async fn run() -> Result<()> {
    let mut server = HttpServer::new(|| App::new().configure(routes));

    //TODO: use config isntead of hardcoded ip address
    server = server.bind("0.0.0.0:8080")?;

    server.run().await
}
