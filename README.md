Rust bindings for the Launchpad API
===================================

Example:

```rust
use launchpadlib::Resource;
use launchpadlib::v1_0::ServiceRoot;

fn main() {
    use url::Url;

    let client = launchpadlib::Client::anonymous("just+testing");
    let service_root = launchpadlib::v1_0::service_root(&client).unwrap();
    let people = service_root.people().unwrap();
    let person = people.get_by_email(&client, "jelmer@jelmer.uk").unwrap();
    let ssh_keys = person.sshkeys(&client).unwrap().map(|k| k.keytext).collect::<Vec<_>>();
    println!("SSH Keys: {:?}", ssh_keys);
}
```

Bindings are generated from the wadl published by Launchpad.

Limitations and bugs
--------------------

* There is only a blocking API available at the moment
* While bindings are generated from the entire WADL file, I have only used a
  small number of them. Please report bugs if you run into issues.
* Launchpad's WADL is incorrect in places, e.g. claiming that certain fields
  are optional while they will actually be set to null. Any problems with the
  WADL will impact the usability of the rust bindings.

  See fixup.xsl for manual patches that are applied; this file is
  almost certainly incomplete.
