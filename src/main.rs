#[allow(warnings, unused)]
mod prisma;

use std::vec;

use anyhow::Result;
use axum::routing::post;
use prisma::{comment, post, PrismaClient};
// use prisma_client_rust::NewClientError;
// use tracing::info;

// use crate::prisma::post;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().expect(".env file not found");

    tracing_subscriber::fmt::init();

    let client = PrismaClient::_builder()
        .with_url(std::env::var("DATABASE_URL").expect("No DATABASE_URL environment variable"))
        .build()
        .await?;

    // DB同期
    // #[cfg(debug_assertions)] // cargo run
    // client._db_push().await?;
    // #[cfg(not(debug_assertions))] // release
    // client._migrate_deploy().await?;

    // 挿入
    // let result = client
    //     .user()
    //     .create(
    //         format!("id{}", chrono::Utc::now().timestamp()).to_string(),
    //         "display_name".to_string(),
    //         "test1".to_string(),
    //         "test2".to_string(),
    //         "test3".to_string(),
    //         vec![],
    //     )
    //     .exec()
    //     .await?;

    // let result = client
    //     .post()
    //     .create(
    //         true,
    //         "title".to_string(),
    //         vec![
    //             post::id::set("id".to_string()),
    //             content::set(Some("content".to_string())),
    //         ],
    //     )
    //     .exec()
    //     .await?;

    // let result = client
    //     .comment()
    //     .delete(comment::id::equals("clv0zl6q70000bkouep3ktem7".to_string()))
    //     .exec()
    //     .await?;

    // let a = client.user().find_many(vec![]).exec().await?;
    // dbg!(a);
    // let a = client
    //     .post()
    //     .find_many(vec![post::id::equals("id".to_string())])
    //     .exec()
    //     .await?;
    // dbg!(a);
    // let a = client.comment().find_many(vec![]).exec().await?;
    // dbg!(a);

    // postに関連したcommentを表示（left join）
    dbg!("postに関連したcommentを表示（left join）");
    let a = client
        .post()
        .find_many(vec![])
        .with(post::comments::fetch(vec![comment::id::equals(
            "clv0z81z10000bkirxeg0zeqf".to_string(),
        )]))
        .exec()
        .await?;
    dbg!(a);

    // コメントがあるものだけ表示
    dbg!("コメントがあるものだけ表示");
    let a = client
        .post()
        .find_many(vec![post::comments::some(vec![])])
        .with(post::comments::fetch(vec![]))
        .exec()
        .await?;

    let a = client
        .post()
        .find_many(vec![post::comments::some(vec![comment::id::equals(
            "clv0z81z10000bkirxeg0zeqf".to_string(),
        )])])
        .with(post::comments::fetch(vec![]))
        .exec()
        .await?;

    dbg!(a);

    Ok(())
}
