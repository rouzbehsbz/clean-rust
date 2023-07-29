use dotenv::dotenv;
use presentation::server::run;

mod __tests__;
mod application;
mod container;
mod domain;
mod infrastructure;
mod presentation;
mod config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    run().await
}
