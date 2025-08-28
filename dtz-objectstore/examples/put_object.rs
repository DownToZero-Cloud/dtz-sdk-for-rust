#[tokio::main]
async fn main() {
    let api_key = std::env::var("DTZ_API_KEY").expect("DTZ_API_KEY must be set");
    let config = dtz::Configuration {
        api_key: Some(api_key),
        ..Default::default()
    };
    let payload = "Hello, world!".as_bytes().to_vec();
    let result = dtz::objectstore::apis::put_object(&config, "file.txt", Some(payload), None).await;
    match result {
        Ok(_) => println!("Uploaded file.txt"),
        Err(e) => println!("Error uploading file.txt: {}", e),
    }
}
