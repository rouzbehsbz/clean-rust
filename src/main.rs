use domain::entities::user::User;
use presentation::server::run;

mod application;
mod common;
mod container;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
