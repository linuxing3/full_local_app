use full_local_app::app::App;
use loco_rs::testing;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_request_find_all() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/articles").await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.text(), "[]");
    })
    .await;
}
#[tokio::test]
#[serial]
async fn can_request_find_first() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/articles/1").await;
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.text(), "{}");
    })
    .await;
}
