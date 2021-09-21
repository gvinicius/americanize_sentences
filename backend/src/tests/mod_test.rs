use crate::{db, http, router};
use hyper::{body::to_bytes, client::HttpConnector, Body, Client as HyperClient, Method, Request};
use hyper_tls::HttpsConnector;
use mock::*;
use std::sync::RwLock;
use warp::test::request;

mod mock;

lazy_static! {
    pub static ref MOCK_HTTP_SERVER: RwLock<WiremockServer> = RwLock::new(WiremockServer::new());
    static ref SERVER: RwLock<Server> = RwLock::new(Server::new());
}

#[tokio::test]
async fn test_list_e2e() {
    setup_wiremock().await;
    init_real_server().await;
    let http_client = http_client();

    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost:8080/conversions")
        .body(Body::empty())
        .unwrap();
    let resp = http_client.request(req).await.unwrap();
    assert_eq!(resp.status(), 200);
    let body_bytes = to_bytes(resp.into_body()).await.unwrap();
    assert_eq!(body_bytes, r#"[]"#);
}

async fn init_real_server() {
    let _ = init_db().await;
    SERVER.write().unwrap().init_server().await;
}

fn http_client() -> HyperClient<HttpsConnector<HttpConnector>> {
    let https = HttpsConnector::new();
    HyperClient::builder().build::<_, Body>(https)
}
