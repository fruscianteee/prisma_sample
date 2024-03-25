#[allow(warnings, unused)]
mod prisma;

use anyhow::Result;
use prisma::PrismaClient;
use prisma_client_rust::NewClientError;
use tracing::info;

use crate::prisma::post;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().expect(".env file not found");
    let client = PrismaClient::_builder()
        .with_url(std::env::var("DATABASE_URL").expect("No DATABASE_URL environment variable"))
        .build()
        .await?;

    // DB同期
    #[cfg(debug_assertions)]
    client._db_push().await?;
    #[cfg(not(debug_assertions))]
    client._migrate_deploy().await?;

    // let a = client
    //     .user()
    //     .create("id".to_string(), "display_name".to_string(), vec![])
    //     .exec()
    //     .await?;

    // dbg!(client.user().find_many(vec![]).exec().await);
    // dbg!("db push");
    // let a = client._db_push().await.unwrap();
    // dbg!(a);
    // dbg!("Running database migrations");
    // client._migrate_deploy().await?;
    // dbg!("Database migrations completed");

    // let a = client.user().find_many(vec![]).exec().await;
    // dbg!(a);
    Ok(())
}
