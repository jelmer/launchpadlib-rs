fn override_type_name(
    container: &wadl::codegen::ParamContainer,
    type_name: &str,
    param_name: &str,
    config: &wadl::codegen::Config,
) -> Option<String> {
    if param_name == "entries" {
        return match container {
            wadl::codegen::ParamContainer::Representation(rd) => match rd.id.as_deref() {
                Some(n) => Some(format!(
                    "Vec<{}>",
                    map_page_to_full(wadl::codegen::camel_case_name(n).as_str())
                )),
                _ => {
                    panic!(
                        "Unknown representation id: {}",
                        rd.id.as_deref().unwrap_or_default()
                    );
                }
            },
            _ => Some("Vec<serde_json::Value>".to_string()),
        };
    }

    match param_name {
        n if n.ends_with("_count") => Some("usize"),
        n if n.ends_with("_url") => Some("url::Url"),
        n if n.starts_with("is_") => Some("bool"),
        "http_etag" => Some("String"),
        "affected" => Some("bool"),
        "scopes" => Some("Vec<String>"),
        "start" | "total_size" => Some("usize"),
        "component_name" => Some("String"),
        "pocket" => Some("String"),
        "title" => Some("String"),
        "authorized_size" => Some("usize"),
        "display_name" | "displayname" => Some("String"),
        "external_dependencies" => Some("String"),
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
        "data" if type_name == "HostedFile" => {
            if config.r#async {
                Some("*reqwest::multipart::Part")
            } else {
                Some("*reqwest::blocking::multipart::Part")
            }
        }
        "deb_version_template" => Some("String"),
        "default_branch" => Some("String"),
        "default_membership_period" => Some("usize"),
        "default_renewal_period" => Some("usize"),
        "dependencies" => Some("Vec<String>"),
        "development_series_alias" => Some("String"),
        "diffstat" => Some("String"),
        "display_arches" => Some("crate::types::PackageUploadArches"),
        "display_version" => Some("String"),
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
        "file_content" => {
            if config.r#async {
                Some("*reqwest::multipart::Part")
            } else {
                Some("*reqwest::blocking::multipart::Part")
            }
        }
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
        "official_bug_tags" => Some("Vec<String>"),
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
        "proposition" => Some("String"),
        "qualifies_for_free_hosting" => Some("bool"),
        "recipe_text" => Some("String"),
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
        "send_notifications" => Some("bool"),
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
        "supports_virtualized" => Some("bool"),
        "suppress_subscription_notifications" => Some("bool"),
        "tag" => Some("String"),
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

fn map_page_to_full(name: &str) -> String {
    format!("{}Full", name.strip_suffix("Page").unwrap())
}

#[test]
fn test_map_page_to_full() {
    assert_eq!(map_page_to_full("FooPage"), "FooFull");
}

fn generate_representation_traits(
    _def: &wadl::ast::RepresentationDef,
    name: &str,
    _representation: &wadl::ast::RepresentationDef,
    config: &wadl::codegen::Config,
) -> Option<Vec<String>> {
    if name.ends_with("Page") {
        let r = map_page_to_full(name);
        let mut ret = vec![];

        if config.r#async {
            ret.push("#[async_trait::async_trait]\n".to_string());
        }

        ret.push(format!(
            "impl {} for {} {{\n",
            if config.r#async {
                "crate::r#async::page::Page"
            } else {
                "crate::blocking::page::Page"
            },
            name
        ));
        ret.push("    type Item = ".to_string() + r.as_str() + ";\n");
        ret.extend(
            if config.r#async {
                vec![
            format!("    async fn next<'a>(&'a self, client: &'a dyn {}) -> std::result::Result<Option<Self>, wadl::Error> {{ if let Some(p) = self.next_collection() {{ Ok(Some(p.get(client).await?)) }} else {{ Ok(None) }} }}\n", config.client_trait_name()),
            format!("    async fn prev<'a>(&'a self, client: &'a dyn {}) -> std::result::Result<Option<Self>, wadl::Error> {{ if let Some(p) = self.prev_collection() {{ Ok(Some(p.get(client).await?)) }} else {{ Ok(None) }} }}\n", config.client_trait_name()),
                ]
            } else {
                vec![
            format!("    fn next<'a>(&'a self, client: &'a dyn {}) -> std::result::Result<Option<Self>, wadl::Error> {{ self.next_collection().map(|x| x.get(client)).transpose() }}\n", config.client_trait_name()),
            format!("    fn prev<'a>(&'a self, client: &'a dyn {}) -> std::result::Result<Option<Self>, wadl::Error> {{ self.prev_collection().map(|x| x.get(client)).transpose() }}\n", config.client_trait_name()),
                ]
            });

        ret.extend(vec![
            "    fn start(&self) -> usize { self.start }\n".to_string(),
            "    fn total_size(&self) -> Option<usize> { self.total_size.into_total_size() }\n"
                .to_string(),
            "    fn entries(&self) -> Vec<".to_string()
                + r.as_str()
                + "> { self.entries.clone() }\n",
            "}\n".to_string(),
        ]);
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

fn extend_accessor(
    param: &wadl::ast::Param,
    accessor_name: &str,
    type_name: &str,
    config: &wadl::codegen::Config,
) -> Vec<String> {
    // if the accessor name ends with _collection, we need to generate a more idiomatic accessor
    if let Some(field_name) = accessor_name.strip_suffix("_collection") {
        // find the bit in between the last < and the first >
        let bn = type_name
            .rfind('<')
            .map(|i| &type_name[i + 1..])
            .unwrap_or(type_name)
            .trim_end_matches('>');
        let inner_type = &bn[bn.rfind(' ').map_or(0, |x| x + 1)..];
        let pr = if let Some(prefix) = inner_type.strip_suffix("PageResource") {
            prefix.to_string()
        } else {
            return vec![];
        };
        let mut lines = vec![];
        if let Some(doc) = param.doc.as_ref() {
            lines.extend(wadl::codegen::generate_doc(doc, 1, config));
        }
        let page_type = match pr.as_str() {
            "People" => "Person".to_string(),
            t if t.ends_with("Countries")
                || t.ends_with("Repositories")
                || t.ends_with("Entries") =>
            {
                format!("{}y", t.strip_suffix("ies").unwrap())
            }
            t if t.ends_with("ieses") => format!("{}ies", t.strip_suffix("ieses").unwrap()),
            "Archives" | "Bugs" | "BugTrackers" | "CharmBases" | "CharmRecipes"
            | "Distributions" | "Builders" | "Languages" | "Cves" | "Projects" | "Processors"
            | "Polls" | "Packagesets" | "Specifications" | "Snaps" | "SnapBases" | "Questions" => {
                pr.strip_suffix('s').unwrap().to_string()
            }
            t if t.ends_with("Blobs") || t.ends_with("Groups") => {
                t.strip_suffix('s').unwrap().to_string()
            }
            "Livefses" => "Livefs".to_string(),
            "Branches" => "Branch".to_string(),
            t => t.to_string(),
        } + "Page";
        let pc_type = if config.r#async {
            "crate::r#async::page::PagedCollection"
        } else {
            "crate::blocking::page::PagedCollection"
        };
        let opt_async = if config.r#async { "async " } else { "" };
        lines.extend(if type_name.starts_with("Option<") {
            vec![
                    format!("    pub {}fn {}<'a>(&'a self, client: &'a dyn {}) -> std::result::Result<Option<{}<'a, {}>>, wadl::Error> {{\n", opt_async, field_name, config.client_trait_name(), pc_type, page_type),
                    if config.r#async {
                        format!("        if let Some(c) = self.{}_collection() {{ Ok(Some({}::new(client, c.get(client).await?))) }} else {{ Ok(None) }}\n", field_name, pc_type)
                    } else {
                        format!("        self.{}_collection().map(|x| Ok({}::new(client, x.get(client)?))).transpose()\n", field_name, pc_type)
                    },
                    format!("    }}\n"),
            ]
        } else {
            vec![
                format!("    pub {}fn {}<'a>(&'a self, client: &'a dyn {}) -> std::result::Result<{}<'a, {}>, wadl::Error> {{\n", opt_async, field_name, config.client_trait_name(), pc_type, page_type),
                if config.r#async {
                    format!("        Ok({}::new(client, self.{}_collection().get(client).await?))\n", pc_type, field_name)
                } else {
                    format!("        Ok({}::new(client, self.{}_collection().get(client)?))\n", pc_type, field_name)
                },
                format!("    }}\n"),
            ]
        });
        lines
    } else {
        vec![]
    }
}

fn extend_method(
    resource_type: &str,
    name: &str,
    ret_type: &str,
    config: &wadl::codegen::Config,
) -> Vec<String> {
    let pc_type = if config.r#async {
        "crate::r#async::page::PagedCollection"
    } else {
        "crate::blocking::page::PagedCollection"
    };
    if !resource_type.ends_with("-page-resource") && name == "get" && ret_type.contains("Page") {
        vec![
            format!("    /// Get a paged collection of {}.\n", ret_type),
            format!("    pub {}fn iter<'a>(&'a self, client: &'a dyn {}) -> std::result::Result<{}<'a, {}>, wadl::Error> {{\n", if config.r#async { "async " } else { "" }, config.client_trait_name(), pc_type, ret_type),
            if config.r#async {
                format!("        Ok({}::new(client, self.get(client).await?))\n", pc_type)
            } else {
                format!("        Ok({}::new(client, self.get(client)?))\n", pc_type)
            },
            format!("    }}\n"),
        ]
    } else {
        vec![]
    }
}

fn map_type_for_response(
    method: &str,
    type_name: &str,
    config: &wadl::codegen::Config,
) -> Option<(String, String)> {
    if !type_name.ends_with("Page") {
        return None;
    }

    if method == "get" {
        return None;
    }

    let pc_type = if config.r#async {
        "crate::r#async::page::PagedCollection"
    } else {
        "crate::blocking::page::PagedCollection"
    };

    Some((
        format!("{}<'a, {}>", pc_type, type_name),
        format!("|x| {}::new(client, x)", pc_type),
    ))
}

fn deprecated_param(param: &wadl::ast::Param) -> bool {
    if let Some(doc) = param.doc.as_ref() {
        doc.content.contains("[DEPRECATED]")
    } else {
        false
    }
}

fn options_enum_name(param: &wadl::ast::Param, exists: Box<dyn Fn(&str) -> bool>) -> String {
    let options = param
        .options
        .as_ref()
        .unwrap()
        .keys()
        .collect::<std::collections::HashSet<_>>();
    let name = match param.name.as_str() {
        "status" => match param.doc.as_ref().unwrap().content.as_str().trim() {
            n if n.contains("The new status of the merge proposal.") => {
                "MergeProposalStatus".to_string()
            }
            n if n.contains("The status of this publishing record") => {
                "PublishingRecordStatus".to_string()
            }
            n if n.contains("Return only items that have this status.") => {
                "PackageUploadStatus".to_string()
            }
            n if n.contains("The state of this membership") => "TeamMembershipStatus".to_string(),
            n if n.contains("The status of this subscription") => {
                "ArchiveSubscriptionStatus".to_string()
            }
            _ if options
                == ["Nominated", "Approved", "Declined"]
                    .into_iter()
                    .collect::<std::collections::HashSet<_>>() =>
            {
                "BugNominationStatus".to_string()
            }
            _ if options
                == ["Completed", "Pending", "Failed"]
                    .into_iter()
                    .collect::<std::collections::HashSet<_>>() =>
            {
                "CharmRecipeStatus".to_string()
            }
            _ if options.contains("Won't Fix") => "BugTaskStatus".to_string(),
            n if n.contains("Whether or not the vulnerability has been reviewed and") => {
                "CveStatus".to_string()
            }
            n if n.contains("The current status of a mirror") => "MirrorStatus".to_string(),
            n if n.contains("The current status of this difference") => {
                "DistroSeriesDifferenceStatus".to_string()
            }
            _ => {
                let mut name = "Status".to_string();
                while exists(name.as_str()) {
                    name.push('_');
                }
                name
            }
        },
        "lifecycle_status" => {
            if param.doc.as_ref().unwrap().content.as_str().contains("Cve") {
                "CveLifecycleStatus".to_string()
            } else if options
                == [
                    "Experimental",
                    "Development",
                    "Mature",
                    "Merged",
                    "Abandoned",
                ]
                .into_iter()
                .collect::<std::collections::HashSet<_>>()
            {
                "BranchLifecycleStatus".to_string()
            } else if options
                == ["Started", "Not started", "Complete"]
                    .into_iter()
                    .collect::<std::collections::HashSet<_>>()
            {
                "SpecificationLifecycleStatus".to_string()
            } else {
                panic!("Unknown lifecycle_status options: {:?}", options);
            }
        }
        "type" => {
            if param
                .doc
                .as_ref()
                .unwrap()
                .content
                .as_str()
                .contains("Attachment Type")
            {
                "AttachmentType".to_string()
            } else {
                "Type".to_string()
            }
        }
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
        }
        "repository_format" => {
            if options.iter().any(|o| o.contains("Bazaar")) {
                "BazaarRepositoryFormat".to_string()
            } else {
                "ArchiveRepositoryFormat".to_string()
            }
        }
        n => wadl::codegen::camel_case_name(n),
    };

    name
}

fn reformat_docstring(text: &str) -> String {
    text.replace("[DEPRECATED]", "")
}

fn convert_to_multipart(type_name: &str, expr: &str) -> Option<String> {
    let inner_type = type_name
        .trim_start_matches("Vec<")
        .trim_end_matches('>')
        .trim_start_matches("Option<")
        .trim_end_matches('>');
    if inner_type == "reqwest::blocking::multipart::Part"
        || inner_type == "reqwest::multipart::Part"
    {
        Some(expr.replace('&', "").replace(".url().to_string()", ""))
    } else {
        None
    }
}

const VERSIONS: &[&str] = &["1.0", "devel", "beta"];

fn main() {
    for is_async in [true, false] {
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
            reformat_docstring: Some(Box::new(reformat_docstring)),
            convert_to_multipart: Some(Box::new(convert_to_multipart)),
            r#async: is_async,
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
            let mut generated = target_dir.join("generated");
            if is_async {
                generated = generated.join("async");
            }
            std::fs::create_dir_all(&generated).unwrap();
            let path = generated.join(format!("{}.rs", version.replace('.', "_")));
            std::fs::write(path, code).unwrap();
        }
    }
}
