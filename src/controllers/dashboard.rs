#![allow(clippy::unused_async)]
use crate::views::dashboard;
use loco_rs::prelude::*;

pub async fn render_home(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    dashboard::home(v)
}

pub fn routes() -> Routes {
    Routes::new().prefix("hello").add("/", get(render_home))
}
