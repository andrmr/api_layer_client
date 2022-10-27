#[macro_use]
extern crate lazy_static;

use api_layer_client::ApiLayerClient;

lazy_static! {
    pub static ref CLIENT: ApiLayerClient = ApiLayerClient::new(
        std::env::var("APIKEY")
            .expect("APIKEY env var with a working API key is required")
            .as_ref()
    );
}

#[tokio::test]
async fn test_get_list() {
    let list = CLIENT.list().await;   
    assert!(list.is_ok(), "Reason: {:?}", list.err()) 
}

#[tokio::test]
async fn test_get_live() {
    let live = CLIENT.live("EUR").await;
    assert!(live.is_ok(), "Reason: {:?}", live.err())
}