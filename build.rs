fn override_type_name(_type_name: &str, param_name: &str) -> Option<String> {
    match param_name {
        n if n.ends_with("_count") => Some("usize"),
        n if n.ends_with("_url") => Some("url::Url"),
        n if n.starts_with("is_") => Some("bool"),
        "http_etag" => Some("String"),
        "description" => Some("String"),
        "scopes" => Some("Vec<String>"),
        "start" | "total_size" => Some("usize"),
        "entries" => Some("Vec<serde_json::Value>"),
        "component_name" => Some("String"),
        "pocket" => Some("String"),
        "title" => Some("String"),
        "authorized_size" => Some("usize"),
        "display_name" | "displayname" => Some("String"),
        "external_dependencies" => Some("String"),
        "name" => Some("String"),
        "private" => Some("bool"),
        "publish" => Some("bool"),
        "reference" => Some("String"),
        "relative_build_score" => Some("f64"),
        "require_virtualization" => Some("bool"),
        "active" => Some("bool"),
        "address" => Some("String"),
        "advertise_by_hash" => Some("bool"),
        "allow_internet" => Some("bool"),
        "allowspoilt" => Some("bool"),
        "architecture_specific" => Some("bool"),
        "architecture_tag" => Some("String"),
        "arch_tag" => Some("String"),
        "as_quoted_email" => Some("String"),
        "auto_build" => Some("bool"),
        "auto_build_channels" => Some("Vec<String>"),
        "backports_not_automatic" => Some("bool"),
        "base_url_aliases" => Some("Vec<url::Url>"),
        "base_version" => Some("debversion::Version"),
        "binary_package_name" => Some("String"),
        "binary_package_version" => Some("debversion::Version"),
        "branch_name" => Some("String"),
        "body_text" => Some("String"),
        "bug_reported_acknowledgement" => Some("bool"),
        "bug_reporting_guidelines" => Some("String"),
        "bug_target_display_name" => Some("String"),
        "bug_target_name" => Some("String"),
        "build_channels" => Some("Vec<String>"),
        "build_daily" => Some("bool"),
        "build_path" => Some("String"),
        "build_snap_channels" => Some("Vec<String>"),
        "build_source_tarball" => Some("url::Url"),
        "bzr_identity" => Some("String"),
        "can_be_cancelled" => Some("bool"),
        "can_be_rescored" => Some("bool"),
        "can_be_retried" => Some("bool"),
        "can_expire" => Some("bool"),
        "can_infer_distro_series" => Some("bool"),
        "can_upload_to_store" => Some("bool"),
        "changelog" => Some("String"),
        "changeslist" => Some("String"),
        "channels" => Some("Vec<String>"),
        "code" => Some("String"),
        "code_name" => Some("String"),
        "comment" => Some("String"),
        "commercial_subscription_is_due" => Some("bool"),
        "commit_message" => Some("String"),
        "commit_sha1" => Some("String"),
        "component_names" => Some("Vec<String>"),
        "conflicts" => Some("Vec<String>"),
        "contact_details" => Some("String"),
        "content" => Some("String"),
        "content_type" => Some("String"),
        "count" => Some("usize"),
        "country_dns_mirror" => Some("String"),
        "custom_file_urls" => Some("Vec<url::Url>"),
        "cvs_module" => Some("String"),
        "cvs_root" => Some("String"),
        "deb_version_template" => Some("String"),
        "default_branch" => Some("String"),
        "default_membership_period" => Some("usize"),
        "default_renewal_period" => Some("usize"),
        "dependencies" => Some("Vec<String>"),
        "development_series_alias" => Some("String"),
        "diffstat" => Some("String"),
        "display_arches" => Some("Vec<String>"),
        "display_version" => Some("String"),
        "distroseries" => Some("String"),
        "distro_series_name" => Some("String"),
        "domain_name" => Some("String"),
        "email" => Some("String"),
        "enabled" => Some("bool"),
        "english_name" => Some("String"),
        "error_message" => Some("String"),
        "error_output" => Some("String"),
        "event_type" => Some("String"),
        "event_types" => Some("Vec<String>"),
        "explicit" => Some("bool"),
        "explicitly_private" => Some("bool"),
        "exported_in_languagepacks" => Some("bool"),
        "failnotes" => Some("String"),
        "features" => Some("Vec<String>"),
        "filename" | "file_extension" => Some("String"),
        "find_all_tags" => Some("bool"),
        "fingerprint" => Some("String"),
        "freshmeat_project" => Some("String"),
        "fullseriesname" => Some("String"),
        "git_identity" => Some("String"),
        "git_path" => Some("String"),
        "git_ref_pattern" => Some("String"),
        "git_refs" => Some("Vec<String>"),
        t if t.starts_with("has_") => Some("bool"),
        "heat" => Some("f64"),
        "hide_email_addresses" => Some("bool"),
        "homepage_content" => Some("Option<String>"),
        "id" => Some("String"),
        "importances" => Some("Vec<String>"),
        t if t.starts_with("include_") => Some("bool"),
        "index_compressors" => Some("Vec<String>"),
        "information_types" => Some("Vec<String>"),
        "iso3166code2" => Some("String"),
        "iso3166code3" => Some("String"),
        "jabberid" => Some("String"),
        "karma" => Some("i64"),
        "keep_binary_files_days" => Some("usize"),
        "keyid" => Some("String"),
        "keytext" => Some("String"),
        "landmarks" => Some("Vec<String>"),
        "language_pack_full_export_requested" => Some("bool"),
        "last_change_comment" => Some("String"),
        "last_scanned_id" => Some("i64"),
        "latest_published_component_name" => Some("String"),
        "latitude" | "longitude" => Some("Option<f64>"),
        "license_approved" => Some("bool"),
        "license_info" => Some("String"),
        "licenses" => Some("Vec<String>"),
        "manual" => Some("String"),
        "merged_revision_id" => Some("String"),
        "merged_revno" => Some("i64"),
        "message" => Some("String"),
        "message_body" => Some("String"),
        "metadata" => Some("String"),
        "metadata_override" => Some("String"),
        "mirror_status_message" => Some("String"),
        "network" => Some("String"),
        "newvalue" => Some("String"),
        "nickname" => Some("String"),
        "number_of_duplicates" => Some("usize"),
        "official" => Some("bool"),
        "official_answers" => Some("bool"),
        "official_blueprints" => Some("bool"),
        "official_bugs" => Some("bool"),
        "official_bug_tags" => Some("bool"),
        "official_candidate" => Some("bool"),
        "official_codehosting" => Some("bool"),
        "official_packages" => Some("bool"),
        "oldvalue" => Some("String"),
        "open_resources" => Some("bool"),
        "other_users_affected_count_with_dupes" => Some("usize"),
        "owner_default" => Some("bool"),
        "package_set_name" => Some("String"),
        "parent_source_version" => Some("String"),
        "path" => Some("String"),
        "payload" => Some("String"),
        "pending" => Some("bool"),
        "permission" => Some("String"),
        "phased_update_percentage" => Some("usize"),
        "plural_expression" => Some("String"),
        "plural_forms" => Some("String"),
        "prerequisite_git_path" => Some("String"),
        "prerequisite_revision_id" => Some("String"),
        "priority" => Some("String"),
        "priority_name" => Some("String"),
        "private_bugs" => Some("bool"),
        "programming_lang" | "programming_language" => Some("String"),
        "project_reviewed" => Some("bool"),
        "properties" => Some("Vec<String>"),
        "proposed_not_automatic" => Some("bool"),
        "proposition" => Some("String"),
        "publish_by_hash" => Some("bool"),
        "qualifies_for_free_hosting" => Some("bool"),
        "recipe_text" => Some("String"),
        "redirect_default_traversal" => Some("String"),
        "redirect_release_uploads" => Some("bool"),
        "release_finder_url_pattern" => Some("String"),
        "release_notes" => Some("String"),
        "remote_bug" => Some("String"),
        "remote_importance" => Some("String"),
        "remote_product" => Some("String"),
        "remote_status" => Some("String"),
        "removal_comment" => Some("String"),
        "require_virtualized" => Some("bool"),
        "restricted_resources" => Some("bool"),
        "results" => Some("Vec<String>"),
        "result_summary" => Some("String"),
        "reviewed" => Some("bool"),
        "reviewed_revid" => Some("String"),
        "reviewer_whiteboard" => Some("String"),
        "review_type" => Some("String"),
        "revision_id" => Some("String"),
        "score" => Some("f64"),
        "section_name" => Some("String"),
        "security_contact" => Some("String"),
        "security_related" => Some("bool"),
        "sequence" => Some("usize"),
        "signing_key_fingerprint" => Some("String"),
        "sourceforge_project" => Some("String"),
        "source_git_path" => Some("String"),
        "source_package_name" => Some("String"),
        "sourcepackagename" => Some("String"),
        "source_package_version" => Some("debversion::Version"),
        "source_revision_id" => Some("String"),
        "source_version" => Some("String"),
        "stages" => Some("Vec<String>"),
        "stale" => Some("bool"),
        "statuses" => Some("Vec<String>"),
        "store_channels" => Some("Vec<String>"),
        "store_name" => Some("String"),
        "store_upload" => Some("String"),
        "store_upload_error_message" => Some("String"),
        "store_upload_error_messages" => Some("Vec<String>"),
        "store_upload_revision" => Some("String"),
        "subject" => Some("String"),
        "successful" => Some("bool"),
        "suite_names" => Some("Vec<String>"),
        "summary" => Some("String"),
        "supported" => Some("bool"),
        "supports_mirrors" => Some("bool"),
        "supports_ppas" => Some("bool"),
        "supports_virtualized" => Some("bool"),
        "suppress_subscription_notifications" => Some("bool"),
        "tag" => Some("String"),
        "tags" => Some("Vec<String>"),
        "target_architectures" => Some("Vec<String>"),
        "target_default" => Some("bool"),
        "target_git_path" => Some("String"),
        "target_revision_id" => Some("String"),
        "team_description" => Some("String"),
        "text" => Some("String"),
        "time_zone" => Some("String"),
        "token" => Some("String"),
        "translation_domain" => Some("String"),
        "unique_key" => Some("String"),
        "unique_name" => Some("String"),
        "uri" => Some("url::Url"),
        "url" => Some("url::Url"),
        "usable_distro_series" => Some("bool"),
        "users_affected_count_with_dupes" => Some("usize"),
        "version" => Some("debversion::Version"),
        "virtualized" => Some("bool"),
        "visible" => Some("bool"),
        "vm_host" => Some("String"),
        "vote_tag" => Some("String"),
        "whatchanged" => Some("String"),
        "whiteboard" => Some("String"),
        "wiki" => Some("String"),
        "wikiname" => Some("String"),
        _ => None,
    }
    .map(|s| s.to_string())
}

fn accessor_rename(param_name: &str, type_name: &str) -> Option<String> {
    if let Some(prefix) = param_name.strip_suffix("_collection_link") {
        if !type_name.contains("PageResource") {
            return Some(prefix.to_string());
        }
    }

    param_name.strip_suffix("_link").map(|param_name| {
        if param_name == "self" {
            "self_"
        } else {
            param_name
        }
        .to_string()
    })
}

fn generate_representation_traits(
    _def: &wadl::ast::RepresentationDef,
    name: &str,
    _representation: &wadl::ast::RepresentationDef,
    _config: &wadl::codegen::Config,
) -> Option<Vec<String>> {
    if name.ends_with("Page") {
        let r = format!("{}Full", name.strip_suffix("Page").unwrap());
        let ret = vec![
            "impl crate::page::Page for ".to_string() + name + " {\n",
            "    type Item = ".to_string() + r.as_str() + ";\n",
            "    fn next<'a>(&'a self, client: &'a dyn wadl::Client) -> std::result::Result<Option<Self>, Error> { self.next_collection().map(|x| x.get(client)).transpose() }\n".to_string(),
            "    fn prev<'a>(&'a self, client: &'a dyn wadl::Client) -> std::result::Result<Option<Self>, Error> { self.prev_collection().map(|x| x.get(client)).transpose() }\n".to_string(),
            "    fn start(&self) -> usize { self.start }\n".to_string(),
            "    fn total_size(&self) -> Option<usize> { self.total_size.as_total_size() }\n".to_string(),
            "    fn entries(&self) -> Vec<".to_string()
                + r.as_str()
                + "> { self.entries.iter().map(|v| serde_json::from_value(v.clone()).unwrap()).collect() }\n",
            "}\n".to_string(),
        ];
        Some(ret)
    } else {
        None
    }
}

fn accessor_visibility(_param_name: &str, param_type: &str) -> Option<String> {
    if param_type.ends_with("PageResource") {
        Some("".to_string())
    } else {
        Some("pub".to_string())
    }
}

fn method_visibility(method_name: &str, return_type: &str) -> Option<String> {
    if !(method_name == "get" && return_type.ends_with("Page")) {
        Some("pub".to_string())
    } else {
        Some("".to_string())
    }
}

fn resource_type_visibility(resource_type_name: &str) -> Option<String> {
    if resource_type_name.ends_with("PageResource") {
        Some("".to_string())
    } else {
        Some("pub".to_string())
    }
}

fn extend_accessor(param: &wadl::ast::Param, accessor_name: &str, type_name: &str, config: &wadl::codegen::Config) -> Vec<String> {
    // if the accessor name ends with _collection, we need to generate a more idiomatic accessor
    if let Some(field_name) = accessor_name.strip_suffix("_collection") {
        // find the bit in between the last < and the first >
        let bn = type_name.rfind('<').map(|i| &type_name[i + 1..]).unwrap_or(type_name).trim_end_matches('>');
        let inner_type = &bn[bn.rfind(' ').map_or(0, |x| x+1)..];
        let pr = if let Some(prefix) = inner_type.strip_suffix("PageResource") {
            prefix.to_string()
        } else {
            return vec![];
        };
        let mut lines = vec![];
        for doc in &param.doc {
            lines.extend(wadl::codegen::generate_doc(doc, 1, config));
        }
        let page_type = match pr.as_str() {
            "People" => "Person".to_string(),
            t if t.ends_with("Countries") || t.ends_with("Repositories") || t.ends_with("Entries") => format!("{}y", t.strip_suffix("ies").unwrap()),
            t if t.ends_with("ieses") => format!("{}ies", t.strip_suffix("ieses").unwrap()),
            "Archives" | "Bugs" | "BugTrackers" | "CharmBases" | "CharmRecipes" | "Distributions" | "Builders" | "Languages" | "Cves" | "Projects" | "Processors" | "Polls" | "Packagesets" | "Specifications" | "Snaps" | "SnapBases" | "Questions" => pr.strip_suffix('s').unwrap().to_string(),
            t if t.ends_with("Blobs") || t.ends_with("Groups") => t.strip_suffix('s').unwrap().to_string(),
            "Livefses" => "Livefs".to_string(),
            "Branches" => "Branch".to_string(),
            t => t.to_string()
        } + "Page";
        lines.extend(if type_name.starts_with("Option<") {
            vec![
                    format!("    pub fn {}<'a>(&'a self, client: &'a dyn wadl::Client) -> std::result::Result<Option<crate::page::PagedCollection<'a, {}>>, Error> {{\n", field_name, page_type),
                    format!("        self.{}_collection().map(|x| Ok(crate::page::PagedCollection::new(client, x.get(client)?))).transpose()\n", field_name),
                    format!("    }}\n"),
            ]
        } else {
            vec![
                format!("    pub fn {}<'a>(&'a self, client: &'a dyn wadl::Client) -> std::result::Result<crate::page::PagedCollection<'a, {}>, Error> {{\n", field_name, page_type),
                format!("        Ok(crate::page::PagedCollection::new(client, self.{}_collection().get(client)?))\n", field_name),
                format!("    }}\n"),
            ]
        });
        lines
    } else {
        vec![]
    }
}

fn extend_method(resource_type: &str, name: &str, ret_type: &str, _config: &wadl::codegen::Config) -> Vec<String> {
    if !resource_type.ends_with("-page-resource") && name == "get" && ret_type.contains("Page") {
        vec![
            format!("    /// Get a paged collection of {}.\n", ret_type),
            format!("    pub fn iter<'a>(&'a self, client: &'a dyn wadl::Client) -> std::result::Result<crate::page::PagedCollection<'a, {}>, Error> {{\n", ret_type),
            format!("        Ok(crate::page::PagedCollection::new(client, self.get(client)?))\n"),
            format!("    }}\n"),
        ]
    } else {
        vec![]
    }
}

fn map_type_for_response(method: &str, type_name: &str) -> Option<(String, String)> {
    if !type_name.ends_with("Page") {
        return None;
    }

    if method == "get" {
        return None;
    }

    Some((format!("crate::page::PagedCollection<'a, {}>", type_name), "|x| crate::page::PagedCollection::new(client, x)".to_string()))
}

fn deprecated_param(param: &wadl::ast::Param) -> bool {
    if let Some(doc) = param.doc.as_ref() {
        doc.content.contains("[DEPRECATED]")
    } else {
        false
    }
}

fn options_enum_name(param: &wadl::ast::Param, exists: Box<dyn Fn(&str) -> bool>) -> String {
    let options = param.options.as_ref().unwrap().keys().collect::<std::collections::HashSet<_>>();
    let name = match param.name.as_str() {
        "status" => {
            match param.doc.as_ref().unwrap().content.as_str().trim() {
                n if n.contains("The new status of the merge proposal.") => "MergeProposalStatus".to_string(),
                n if n.contains("The status of this publishing record") => "PublishingRecordStatus".to_string(),
                n if n.contains("Return only items that have this status.") => "PackageUploadStatus".to_string(),
                n if n.contains("The state of this membership") => "TeamMembershipStatus".to_string(),
                n if n.contains("The status of this subscription") => "ArchiveSubscriptionStatus".to_string(),
                _ if options == ["Nominated", "Approved", "Declined"].into_iter().collect::<std::collections::HashSet<_>>() => "BugNominationStatus".to_string(),
                _ if options == ["Completed", "Pending", "Failed"].into_iter().collect::<std::collections::HashSet<_>>() => "CharmRecipeStatus".to_string(),
                _ if options.contains("Won't Fix") => "BugTaskStatus".to_string(),
                n if n.contains("Whether or not the vulnerability has been reviewed and") => "CveStatus".to_string(),
                n if n.contains("The current status of a mirror") => "MirrorStatus".to_string(),
                n if n.contains("The current status of this difference") => "DistroSeriesDifferenceStatus".to_string(),
                _ => {
                    let mut name = "Status".to_string();
                    while exists(name.as_str()) {
                        name.push('_');
                    }
                    name
                }
            }
        }
        "lifecycle_status" => {
            if param.doc.as_ref().unwrap().content.as_str().contains("Cve") {
                "CveLifecycleStatus".to_string()
            } else if options == ["Experimental", "Development", "Mature", "Merged", "Abandoned"].into_iter().collect::<std::collections::HashSet<_>>() {
                "BranchLifecycleStatus".to_string()
            } else if options == ["Started", "Not started", "Complete"].into_iter().collect::<std::collections::HashSet<_>>() {
                "SpecificationLifecycleStatus".to_string()
            } else {
                panic!("Unknown lifecycle_status options: {:?}", options);
            }
        },
        "type" => {
            if param.doc.as_ref().unwrap().content.as_str().contains("Attachment Type") {
                "AttachmentType".to_string()
            } else {
                "Type".to_string()
            }
        },
        "order_by" => {
            if options.contains("by branch name") {
                "BranchOrderBy".to_string()
            } else if options.contains("by repository name") {
                "RepositoryOrderBy".to_string()
            } else if options.contains("by opening date") {
                "PollOrderBy".to_string()
            } else {
                panic!("Unknown order_by options: {:?}", options);
            }
        },
        "repository_format" => {
            if options.iter().any(|o| o.contains("Bazaar")) {
                "BazaarRepositoryFormat".to_string()
            } else {
                "ArchiveRepositoryFormat".to_string()
            }
        }
        n => wadl::codegen::camel_case_name(n)
    };

    name
}

const VERSIONS: &[&str] = &["1.0", "devel", "beta"];

fn main() {
    #[allow(clippy::needless_update)]
    let config = wadl::codegen::Config {
        override_type_name: Some(Box::new(override_type_name)),
        param_accessor_rename: Some(Box::new(accessor_rename)),
        generate_representation_traits: Some(Box::new(generate_representation_traits)),
        strip_code_examples: true,
        accessor_visibility: Some(Box::new(accessor_visibility)),
        resource_type_visibility: Some(Box::new(resource_type_visibility)),
        extend_accessor: Some(Box::new(extend_accessor)),
        extend_method: Some(Box::new(extend_method)),
        method_visibility: Some(Box::new(method_visibility)),
        map_type_for_response: Some(Box::new(map_type_for_response)),
        deprecated_param: Some(Box::new(deprecated_param)),
        options_enum_name: Some(Box::new(options_enum_name)),
        ..Default::default()
    };

    for version in VERSIONS {
        let wadl = if let Ok(text) = std::fs::read_to_string(format!(
            "{}/wadl/{}.wadl",
            env!("CARGO_MANIFEST_DIR"),
            version
        )) {
            text
        } else {
            let url = format!("https://api.launchpad.net/{}/", version);
            reqwest::blocking::Client::new()
                .request(reqwest::Method::GET, &url)
                .header("Accept", "application/vd.sun.wadl+xml")
                .send()
                .unwrap()
                .error_for_status()
                .unwrap()
                .text()
                .unwrap()
        };

        let wadl_app = wadl::parse_string(wadl.as_str()).unwrap();
        let code = wadl::codegen::generate(&wadl_app, &config);
        let target_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
            .canonicalize()
            .unwrap();
        let generated = target_dir.join("generated");
        std::fs::create_dir_all(&generated).unwrap();
        let path = generated.join(format!("{}.rs", version.replace('.', "_")));
        std::fs::write(path, code).unwrap();
    }
}
