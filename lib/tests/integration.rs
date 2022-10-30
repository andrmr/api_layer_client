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
    let live = CLIENT.live("EUR", None).await;
    assert!(live.is_ok(), "Reason: {:?}", live.err())
}

#[tokio::test]
async fn test_get_live_with_currencies() {
    let currencies = vec!["USD".to_string(), "RON".to_string()];
    let live = CLIENT.live("EUR", Some(&currencies)).await;
    assert!(live.is_ok(), "Reason: {:?}", live.err())
}
