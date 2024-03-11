use eyre::Context;
use full_local_app::app::App;
use full_local_app::models::_entities::articles;
#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let _ctx = playground::<App>().await.context("playground")?;

    let active_model: articles::ActiveModel = articles::ActiveModel {
        title: Set(Some("how to build apps in 3 steps".to_string())),
        content: Set(Some("use Loco: https://loco.rs".to_string())),
        ..Default::default()
    };

    active_model.insert(&_ctx.db).await.unwrap();

    let res = articles::Entity::find().all(&_ctx.db).await.unwrap();
    println!("{:?}", res);

    println!("welcome to playground. edit me at `examples/playground.rs`");

    Ok(())
}
