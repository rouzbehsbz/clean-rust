use presentation::server::run;

mod application;
mod common;
mod domain;
mod infrastructure;
mod presentation;
mod container;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
