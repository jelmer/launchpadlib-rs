use launchpadlib::auth::authorize_token_url;
use launchpadlib::blocking::client::auth::{exchange_request_token, get_request_token};
use launchpadlib::blocking::Client;

fn main() {
    // Step 0: Pick a consumer key
    let consumer_key = "your_consumer_key";

    let instance = "launchpad.net";

    // Step 1: Get a request token
    let req_token = get_request_token(instance, consumer_key).unwrap();

    // Step 2: Get the user to authorize the request token
    let auth_url = authorize_token_url(instance, req_token.0.as_str(), None).unwrap();

    println!("Please authorize the request token at {}", auth_url);
    println!("Once done, press enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Step 3: Exchange the request token for an access token
    let access_token = exchange_request_token(
        instance,
        consumer_key,
        None,
        req_token.0.as_str(),
        Some(req_token.1.as_str()),
    )
    .unwrap();

    let client = Client::from_tokens(
        consumer_key,
        None,
        access_token.0.as_str(),
        access_token.1.as_str(),
    );

    let root = launchpadlib::blocking::v1_0::service_root(&client).unwrap();
    let person = root.me().unwrap().get(&client).unwrap();
    println!("{:?}", person);
}
