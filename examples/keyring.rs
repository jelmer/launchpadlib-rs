fn main() {
    let lp = launchpadlib::Client::authenticated(None, "example-keyring").unwrap();
    let root = launchpadlib::v1_0::service_root(&lp).unwrap();
    let person = root.me().unwrap().get(&lp).unwrap();
    println!("{:?}", person);
}
