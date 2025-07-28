#[cfg(all(feature = "api-v1_0", feature = "blocking"))]
mod v1_0_tests {
    use crate::blocking::v1_0::*;
    use url::Url;

    #[test]
    fn test_person_url() {
        let person = Person(Url::parse("https://api.launchpad.net/1.0/~person").unwrap());
        assert_eq!(
            person.url(),
            &Url::parse("https://api.launchpad.net/1.0/~person").unwrap()
        );
    }

    #[test]
    fn test_team_url() {
        let team = Team(Url::parse("https://api.launchpad.net/1.0/~team").unwrap());
        assert_eq!(
            team.url(),
            &Url::parse("https://api.launchpad.net/1.0/~team").unwrap()
        );
    }

    #[test]
    fn test_bug_url() {
        let bug = Bug(Url::parse("https://api.launchpad.net/1.0/bugs/1").unwrap());
        assert_eq!(
            bug.url(),
            &Url::parse("https://api.launchpad.net/1.0/bugs/1").unwrap()
        );
    }

    #[test]
    fn test_parse_person_full() {
        let json = include_str!("../testdata/person.json");
        let person: PersonFull = serde_json::from_str(json).unwrap();
        assert_eq!(person.name, "jelmer");
        assert_eq!(person.display_name, "Jelmer Vernooij");
        assert_eq!(person.time_zone, "Europe/London");
        assert_eq!(person.is_team, false);
        assert!(person.karma > 2000);
        assert_eq!(person.is_ubuntu_coc_signer, true);
        assert_eq!(person.hide_email_addresses, true);
        assert_eq!(person.visibility, "Public");
        assert_eq!(person.self_link.as_ref().unwrap(), "https://api.launchpad.net/1.0/~jelmer");
        assert_eq!(person.web_link.as_ref().unwrap(), "https://launchpad.net/~jelmer");
    }

    #[test]
    fn test_parse_team_full() {
        let json = include_str!("../testdata/team-ubuntu-security.json");
        let team: TeamFull = serde_json::from_str(json).unwrap();
        assert_eq!(team.name, "ubuntu-security");
        assert_eq!(team.display_name, "Ubuntu Security Team");
        assert_eq!(team.is_team, true);
        assert_eq!(team.membership_policy, "Restricted Team");
        assert_eq!(team.members_collection_size, 22);
        assert_eq!(team.active_members_collection_size, 22);
        assert_eq!(team.team_owner_link, "https://api.launchpad.net/1.0/~ubuntu-security-leaders");
    }

    #[test]
    fn test_parse_bug_full() {
        let json = include_str!("../testdata/bug.json");
        let bug: BugFull = serde_json::from_str(json).unwrap();
        assert_eq!(bug.id, 1);
        assert_eq!(bug.title, "Microsoft has a majority market share");
        assert_eq!(bug.name, Some("liberation".to_string()));
        assert_eq!(bug.private, false);
        assert_eq!(bug.information_type, "Public");
        assert_eq!(bug.security_related, false);
        assert!(bug.heat > 10000);
        assert!(bug.message_count > 1900);
        assert!(bug.users_affected_count > 2400);
        assert!(bug.tags.contains(&"microsoft".to_string()));
    }

    #[test]
    fn test_archive_url() {
        let archive = Archive(Url::parse("https://api.launchpad.net/1.0/~user/+archive/ubuntu/ppa").unwrap());
        assert_eq!(
            archive.url(),
            &Url::parse("https://api.launchpad.net/1.0/~user/+archive/ubuntu/ppa").unwrap()
        );
    }

    #[test]
    fn test_distribution_url() {
        let distro = Distribution(Url::parse("https://api.launchpad.net/1.0/ubuntu").unwrap());
        assert_eq!(
            distro.url(),
            &Url::parse("https://api.launchpad.net/1.0/ubuntu").unwrap()
        );
    }

    #[test]
    fn test_project_url() {
        let project = Project(Url::parse("https://api.launchpad.net/1.0/launchpad").unwrap());
        assert_eq!(
            project.url(),
            &Url::parse("https://api.launchpad.net/1.0/launchpad").unwrap()
        );
    }

    #[test]
    fn test_source_package_url() {
        let pkg = SourcePackage(Url::parse("https://api.launchpad.net/1.0/ubuntu/+source/hello").unwrap());
        assert_eq!(
            pkg.url(),
            &Url::parse("https://api.launchpad.net/1.0/ubuntu/+source/hello").unwrap()
        );
    }

    #[test]
    fn test_branch_url() {
        let branch = Branch(Url::parse("https://api.launchpad.net/1.0/~user/project/branch").unwrap());
        assert_eq!(
            branch.url(),
            &Url::parse("https://api.launchpad.net/1.0/~user/project/branch").unwrap()
        );
    }

    #[test]
    fn test_milestone_url() {
        let milestone = Milestone(Url::parse("https://api.launchpad.net/1.0/project/+milestone/1.0").unwrap());
        assert_eq!(
            milestone.url(),
            &Url::parse("https://api.launchpad.net/1.0/project/+milestone/1.0").unwrap()
        );
    }

    #[test]
    fn test_bug_task_url() {
        let bug_task = BugTask(Url::parse("https://api.launchpad.net/1.0/ubuntu/+source/hello/+bug/1").unwrap());
        assert_eq!(
            bug_task.url(),
            &Url::parse("https://api.launchpad.net/1.0/ubuntu/+source/hello/+bug/1").unwrap()
        );
    }

    #[test]
    fn test_specification_url() {
        let spec = Specification(Url::parse("https://api.launchpad.net/1.0/project/+spec/spec-name").unwrap());
        assert_eq!(
            spec.url(),
            &Url::parse("https://api.launchpad.net/1.0/project/+spec/spec-name").unwrap()
        );
    }
}

#[cfg(all(feature = "api-devel", feature = "blocking"))]
mod devel_tests {
    use crate::blocking::devel::*;
    use url::Url;

    #[test]
    fn test_person_url() {
        let person = Person(Url::parse("https://api.launchpad.net/devel/~person").unwrap());
        assert_eq!(
            person.url(),
            &Url::parse("https://api.launchpad.net/devel/~person").unwrap()
        );
    }

    #[test]
    fn test_team_url() {
        let team = Team(Url::parse("https://api.launchpad.net/devel/~team").unwrap());
        assert_eq!(
            team.url(),
            &Url::parse("https://api.launchpad.net/devel/~team").unwrap()
        );
    }

    #[test]
    fn test_bug_url() {
        let bug = Bug(Url::parse("https://api.launchpad.net/devel/bugs/1").unwrap());
        assert_eq!(
            bug.url(),
            &Url::parse("https://api.launchpad.net/devel/bugs/1").unwrap()
        );
    }
}

#[cfg(all(feature = "api-beta", feature = "blocking"))]
mod beta_tests {
    use crate::blocking::beta::*;
    use url::Url;

    #[test]
    fn test_person_url() {
        let person = Person(Url::parse("https://api.launchpad.net/beta/~person").unwrap());
        assert_eq!(
            person.url(),
            &Url::parse("https://api.launchpad.net/beta/~person").unwrap()
        );
    }

    #[test]
    fn test_team_url() {
        let team = Team(Url::parse("https://api.launchpad.net/beta/~team").unwrap());
        assert_eq!(
            team.url(),
            &Url::parse("https://api.launchpad.net/beta/~team").unwrap()
        );
    }

    #[test]
    fn test_bug_url() {
        let bug = Bug(Url::parse("https://api.launchpad.net/beta/bugs/1").unwrap());
        assert_eq!(
            bug.url(),
            &Url::parse("https://api.launchpad.net/beta/bugs/1").unwrap()
        );
    }
}

#[cfg(test)]
mod type_tests {
    use crate::types::PackageUploadArches;

    #[test]
    fn test_package_upload_arches_source() {
        let json = r#""source""#;
        let arches: PackageUploadArches = serde_json::from_str(json).unwrap();
        assert_eq!(arches, PackageUploadArches::Source);
    }

    #[test]
    fn test_package_upload_arches_sync() {
        let json = r#""sync""#;
        let arches: PackageUploadArches = serde_json::from_str(json).unwrap();
        assert_eq!(arches, PackageUploadArches::Sync);
    }

    #[test]
    fn test_package_upload_arches_single() {
        let json = r#""amd64""#;
        let arches: PackageUploadArches = serde_json::from_str(json).unwrap();
        assert_eq!(arches, PackageUploadArches::Arch("amd64".to_string()));
    }

    #[test]
    fn test_package_upload_arches_multiple() {
        let json = r#"["amd64", "i386"]"#;
        let arches: PackageUploadArches = serde_json::from_str(json).unwrap();
        assert_eq!(arches, PackageUploadArches::Arches(vec!["amd64".to_string(), "i386".to_string()]));
    }
}