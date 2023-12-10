fn main() {
    let client = reqwest::blocking::Client::new();
    let root = launchpadlib::v1_0::service_root(&client).unwrap();
    let people = root.people().unwrap();
    let person = people.get_by_email(&client, "jelmer@jelmer.uk").unwrap();
    let ssh_keys = person.sshkeys(&client).unwrap().map(|k| k.unwrap()).collect::<Vec<_>>();
    for key in ssh_keys {
        println!("{} {} {}", match key.keytype.as_str() {
            "RSA" => "ssh-rsa",
            "DSA" => "ssh-dss",
            "ECDSA" => "ecdsa-sha2-nistp256",
            "ED25519" => "ssh-ed25519",
            _ => panic!("Unknown key type {}", key.keytype)
        }, key.keytext, key.comment.trim());
    }
}
