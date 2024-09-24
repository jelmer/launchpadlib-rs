fn main() {
    use launchpadlib::v1_0::Keytype::*;
    let client = reqwest::blocking::Client::new();
    let root = launchpadlib::v1_0::service_root(&client).unwrap();
    let people = root.people().unwrap();
    let person = people.get_by_email(&client, "jelmer@jelmer.uk").unwrap();
    let ssh_keys = person
        .sshkeys(&client)
        .unwrap()
        .map(|k| k.unwrap())
        .collect::<Vec<_>>();
    for key in ssh_keys {
        println!(
            "{} {} {}",
            match key.keytype {
                RSA => "ssh-rsa",
                DSA => "ssh-dss",
                ECDSA => "ecdsa-sha2-nistp256",
                ED25519 => "ssh-ed25519",
            },
            key.keytext,
            key.comment.trim()
        );
    }
}
