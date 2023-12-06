Rust bindings for the Launchpad API
===================================

Example:

```rust
use launchpadlib::Resource;
use launchpadlib::v1_0::ServiceRoot;

fn main() {
    let url = "https://api.launchpad.net/v1_0/".parse().unwrap();
    let root = launchpadlib::devel::get_service_root_by_url(&url).unwrap();
    let people = root
        .get()
        .unwrap()
        .people()
        .unwrap()
        .unwrap();
    let person = people.get_by_email("jelmer@jelmer.uk").unwrap();
    println!("{:?}", person);
}
```

Bindings are generated from the wadl published by Launchpad.

Limitations
-----------

* There is only a blocking API available at the moment

Bugs
----

While bindings are generated from the entire WADL file, I have only
used a small number of them. Please report bugs if you run into issues.

Launchpad's WADL is incorrect in places, e.g. claiming that certain fields
are optional while they will actually be set to null. See fixup.xsl for manual
patches that are applied.
