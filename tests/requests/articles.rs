use full_local_app::app::App;
use loco_rs::testing;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_request_find_all() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/articles").await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.text(), "[{\"created_at\":\"2024-03-11T00:58:53.973576\",\"updated_at\":\"2024-03-11T00:58:53.973576\",\"id\":1,\"title\":\"how to build apps in 3 steps\",\"content\":\"use Loco: https://loco.rs\"},{\"created_at\":\"2024-03-11T01:20:24.843516\",\"updated_at\":\"2024-03-11T01:20:24.843516\",\"id\":2,\"title\":\"how to build apps in 3 steps\",\"content\":\"use Loco: https://loco.rs\"}]");
    })
    .await;
}
#[tokio::test]
#[serial]
async fn can_request_find_first() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/articles/1").await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.text(), "{\"created_at\":\"2024-03-11T00:58:53.973576\",\"updated_at\":\"2024-03-11T00:58:53.973576\",\"id\":1,\"title\":\"how to build apps in 3 steps\",\"content\":\"use Loco: https://loco.rs\"}");
    })
    .await;
}
