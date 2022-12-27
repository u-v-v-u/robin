mod mongo;
mod routing;
mod schema;

use std::net::SocketAddr;

use anyhow::Result;
use dotenvy::dotenv;
use tracing::warn;
use tracing_subscriber::prelude::*;
use warp::Filter;

use mongo::DbClient;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init(); // Print to the Terminal
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer());
    load_env();

    let db = DbClient::new().await?;

    let addr = "127.0.0.1:8787".parse::<SocketAddr>()?;

    let root = warp::get().map(warp::reply);

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not set CTRL-C handler");
        warn!("Received Termination Signal...");
        std::process::exit(0)
    });

    warp::serve(root).run(addr).await;

    Ok(())
}

fn load_env() {
    match dotenv() {
        Ok(_) => {}
        Err(_) => {
            warn!("Something went wrong while loading the .env file")
        }
    }
}
