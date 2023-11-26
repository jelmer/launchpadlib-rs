fn main() {
    let url = "https://api.launchpad.net/1.0/".parse().unwrap();
    let root = launchpadlib::v1_0::get_service_root_by_url(&url).unwrap();
    let people = root
        .get()
        .unwrap()
        .people()
        .unwrap()
        .unwrap();
    let person = people.get_by_email("jelmer@jelmer.uk").unwrap();
    println!("{:?}", person);
}
