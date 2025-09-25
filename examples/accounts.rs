use coinbase_api::app::auth::CoinbaseAuth;
use coinbase_api::app::client::CoinbaseAppClient;

#[tokio::main]
async fn main() {
    let auth = CoinbaseAuth::ApiKeys {
        api_key: String::from("<api-key>"),
        secret_key: String::from("<secret-key>"),
    };
    let client = CoinbaseAppClient::new(auth).unwrap();

    let accounts = client.accounts().await.unwrap();

    for account in accounts {
        println!("{:#?}", account);
    }
}
