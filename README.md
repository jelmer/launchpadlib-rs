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

Feature Flags
-------------

The library supports feature flags for Launchpad's different "pillars" to reduce build times:

* `bugs` - Bug tracking functionality (Bug, CVE, etc.)
* `answers` - Q&A/FAQ functionality (Question, FAQ, etc.)
* `blueprints` - Specification/blueprint functionality
* `code` - Code hosting functionality (Git, Branches, Merge Proposals, etc.)
* `translations` - Translation/localization functionality (POFiles, POTemplates, etc.)
* `packages` - Package management functionality (Archives, Builds, Snaps, etc.)
* `vulnerabilities` - Vulnerability tracking functionality (Vulnerability resources, CVSS scoring, etc.)

By default, all pillar features are enabled. To reduce build times, you can disable features you don't need:

```toml
[dependencies]
launchpadlib = { version = "0.5", default-features = false, features = ["blocking", "api-v1_0", "bugs"] }
```

This can reduce build times significantly.

Note: Resources that span multiple pillars (e.g., linking bugs to branches) require all relevant pillar features to be enabled.

Limitations and bugs
--------------------

* While bindings are generated from the entire WADL file, I have only used a
  small number of them. Please report bugs if you run into issues.
* Launchpad's WADL is incorrect in places, e.g. claiming that certain fields
  are optional while they will actually be set to null. Any problems with the
  WADL will impact the usability of the rust bindings.

  See fixup.xsl for manual patches that are applied; this file is
  almost certainly incomplete.
