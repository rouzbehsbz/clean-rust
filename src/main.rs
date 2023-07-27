use presentation::server::run;

mod application;
mod container;
mod domain;
mod infrastructure;
mod presentation;
mod config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
