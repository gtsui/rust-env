use std::thread;
use std::time::Duration;
use ethers::prelude::*;
use async_recursion::async_recursion;


#[async_recursion]
pub async fn get_ws_provider(url: &str) -> Provider<Ws> {

  let result = Provider::<Ws>::connect(url).await;
  let provider = match result {
    Ok(provider) => {
      provider
    },
    Err(_) => {
      println!("Error: Retrying to connect provider after 5 seconds...");
      thread::sleep(Duration::from_millis(5000));
      get_ws_provider(url).await
    }
  };
  provider
}

pub fn get_http_provider(url: &str) -> Provider<Http> {
  Provider::<Http>::try_from(url).unwrap()
}
