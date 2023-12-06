fn main() {
    let url = "https://api.launchpad.net/1.0/".parse().unwrap();
    let client = reqwest::blocking::Client::new();
    let root = launchpadlib::v1_0::get_service_root_by_url(&url).unwrap();
    let people = root
        .get(&client)
        .unwrap()
        .people()
        .unwrap();
    let person = people.get_by_email(&client, "jelmer@jelmer.uk").unwrap();
    println!("{:?}", person);
}
