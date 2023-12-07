fn main() {
    let client = reqwest::blocking::Client::new();
    let root = launchpadlib::v1_0::service_root(&client).unwrap();
    let people = root
        .people()
        .unwrap();
    let person = people.get_by_email(&client, "jelmer@jelmer.uk").unwrap();
    println!("{:?}", person);
}
