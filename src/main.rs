#![feature(async_fn_in_trait)]

use presentation::server::run;

mod application;
mod common;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
