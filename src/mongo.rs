use std::env;

use mongodb::Client;
use anyhow::Result;

pub struct DbClient(Client);

impl DbClient {
  pub async fn new() -> Result<Self> {
    let uri = env::var("ROBIN_MONGODB_URI")?;
    let client = Client::with_uri_str(uri).await?;
  
    Ok(Self(client))
  }

  pub async fn list(&self) -> Result<()>{
    let shit = self.0.database("robin").list_collection_names(None).await?;

    println!("{shit:?}");

    Ok(())
  }
}
