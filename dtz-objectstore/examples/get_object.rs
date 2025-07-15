#[tokio::main]
async fn main() {
    let api_key = std::env::var("DTZ_API_KEY").expect("DTZ_API_KEY must be set");
    let config = dtz::Configuration {
        api_key: Some(api_key),
        ..Default::default()
    };
    let result = dtz::objectstore::apis::get_object(&config, "file.txt").await;
    match result {
        Ok(response) => {
            let bytes = response.bytes().await.unwrap();
            let string = String::from_utf8(bytes.to_vec()).unwrap();
            println!("response content: {}", string);
        }
        Err(e) => println!("Error uploading file.txt: {}", e),
    }
}
