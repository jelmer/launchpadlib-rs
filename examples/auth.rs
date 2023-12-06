use launchpadlib::auth::{get_request_token, authorize_token_url, exchange_request_token};
use launchpadlib::Client;
use url::Url;

fn main() {
    // Step 0: Pick a consumer key
    let consumer_key = "your_consumer_key";

    // Step 1: Get a request token
    let req_token = get_request_token(consumer_key).unwrap();

    // Step 2: Get the user to authorize the request token
    let auth_url = authorize_token_url(req_token.0.as_str(), None).unwrap();

    println!("Please authorize the request token at {}", auth_url);
    println!("Once done, press enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Step 3: Exchange the request token for an access token
    let access_token = exchange_request_token(consumer_key, None, req_token.0.as_str(), Some(req_token.1.as_str())).unwrap();

    let url: Url = "https://api.launchpad.net/1.0/"
        .parse()
        .unwrap();

    let client = Client::authenticated(consumer_key, None, access_token.0.as_str(), access_token.1.as_str()).unwrap();

    let root = launchpadlib::v1_0::get_service_root_by_url(&url).unwrap();
    let person = root
        .get(&client)
        .unwrap()
        .me()
        .unwrap()
        .get(&client)
        .unwrap();
    println!("{:?}", person);

}
