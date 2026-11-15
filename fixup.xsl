<!-- XSLT Patch -->
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:wadl="http://research.sun.com/wadl/2006/10" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:fix="urn:launchpadlib-rs:fixup" exclude-result-prefixes="fix">
  <!-- Identity template to copy all nodes and attributes unchanged -->
  <xsl:template match="@* | node()">
    <xsl:copy>
      <xsl:apply-templates select="@* | node()"/>
    </xsl:copy>
  </xsl:template>

  <!--
    Attributes to override on params of top-level representations. Each
    entry applies its attributes to every param listed in "names" that
    appears in one of the representations listed in "reps" ("*" matches
    all representations).
  -->
  <fix:params>
    <!-- Launchpad marks every field of a -full representation as required,
         even those that can be null -->
    <fix:param required="false"
      reps="*"
      names="bug_reported_acknowledgement bug_reporting_guidelines"/>
    <fix:param required="false"
      reps="access_token-full"
      names="revoked_by_link date_expires date_last_used"/>
    <fix:param required="false"
      reps="archive-full"
      names="external_dependencies signing_key_fingerprint
             authorized_size description metadata_overrides"/>
    <fix:param required="false"
      reps="archive_dependency-full"
      names="archive_link snap_base_link"/>
    <fix:param required="false"
      reps="archive_subscriber-full"
      names="date_expires"/>
    <fix:param required="false"
      reps="branch-full"
      names="code_import_link sourcepackage_link project_link
             reviewer_link url last_mirror_attempt last_mirrored
             last_scanned last_scanned_id mirror_status_message
             whiteboard"/>
    <fix:param required="false"
      reps="branch_merge_proposal-full"
      names="source_branch_link target_branch_link
             prerequisite_branch_link source_git_repository_link
             target_git_repository_link prerequisite_git_repository_link
             source_git_path target_git_path prerequisite_git_path
             preview_diff_link reviewer_link merge_reporter_link
             superseded_by_link supersedes_link date_merged
             date_review_requested date_reviewed merged_revision_id
             merged_revno reviewed_revid commit_message description"/>
    <fix:param required="false"
      reps="bug-full"
      names="duplicate_of_link latest_patch_uploaded
             who_made_private_link date_made_private name
             date_last_message lock_reason"/>
    <fix:param required="false"
      reps="bug_activity-full"
      names="message oldvalue newvalue"/>
    <fix:param required="false"
      reps="bug_nomination-full"
      names="date_decided decider_link distroseries_link
             productseries_link"/>
    <fix:param required="false"
      reps="bug_task-full"
      names="milestone_link assignee_link bug_watch_link date_assigned
             date_incomplete date_closed date_fix_released date_left_new
             date_left_closed date_fix_committed date_confirmed
             date_in_progress date_triaged date_deferred
             importance_explanation status_explanation"/>
    <fix:param required="false"
      reps="bug_watch-full"
      names="date_last_changed date_last_checked date_next_checked
             remote_importance remote_status"/>
    <fix:param required="false"
      reps="build-full build_farm_job-full ci_build-full
            charm_recipe_build-full craft_recipe_build-full
            livefs_build-full oci_recipe_build-full
            rock_recipe_build-full snap_build-full
            source_package_recipe_build-full"
      names="builder_link build_log_url upload_log_url score duration
             date_first_dispatched date_started datebuilt dependencies"/>
    <fix:param required="false"
      reps="build-full"
      names="buildinfo_url changesfile_url"/>
    <fix:param required="false"
      reps="charm_recipe_build-full craft_recipe_build-full
            rock_recipe_build-full snap_build-full"
      names="revision_id store_upload_error_message
             store_upload_revision"/>
    <fix:param required="false"
      reps="livefs_build-full"
      names="unique_key version metadata_override"/>
    <fix:param required="false"
      reps="oci_recipe_build-full"
      names="date eta registry_upload_error_summary registry_upload_errors"/>
    <fix:param required="false"
      reps="snap_build-full"
      names="snap_base_link store_upload_error_messages"/>
    <fix:param required="false"
      reps="charm_recipe_build_request-full
            craft_recipe_build_request-full
            oci_recipe_build_request-full rock_recipe_build_request-full
            snap_build_request-full"
      names="date_finished error_message"/>
    <fix:param required="false"
      reps="builder-full"
      names="current_build_link failnotes vm_host version open_resources
             restricted_resources"/>
    <fix:param required="false"
      reps="charm_recipe-full"
      names="git_repository_link git_repository_url git_ref_link
             git_path"/>
    <fix:param required="false"
      reps="charm_recipe-full craft_recipe-full oci_recipe-full
            rock_recipe-full snap-full"
      names="build_path store_name"/>
    <fix:param required="false"
      reps="code_import-full"
      names="branch_link git_repository_link url cvs_root cvs_module
             date_last_successful"/>
    <fix:param required="false"
      reps="code_review_comment-full"
      names="date_deleted date_last_edited vote_tag"/>
    <fix:param required="false"
      reps="code_review_vote_reference-full"
      names="comment_link review_type"/>
    <fix:param required="false"
      reps="cve-full"
      names="date_made_public discovered_by cvss metadata"/>
    <fix:param required="false"
      reps="distribution-full"
      names="oci_project_admin_link commercial_subscription_link
             bug_supervisor_link driver_link code_admin_link
             security_admin_link current_series_link
             translationgroup_link main_archive_link
             development_series_alias"/>
    <fix:param required="false"
      reps="distribution_mirror-full"
      names="date_reviewed reviewer_link ftp_base_url http_base_url
             https_base_url rsync_base_url whiteboard"/>
    <fix:param required="false"
      reps="distribution_source_package-full"
      names="upstream_product_link"/>
    <fix:param required="false"
      reps="distro_arch_series-full"
      names="chroot_url underlying_architecture_tag"/>
    <fix:param required="false"
      reps="distro_series-full"
      names="datereleased driver_link parent_series_link
             nominatedarchindep_link previous_series_link changeslist
             main_archive_link"/>
    <fix:param required="false"
      reps="distro_series_difference-full"
      names="base_version source_version parent_source_version
             package_diff_url parent_package_diff_url"/>
    <fix:param required="false"
      reps="faq-full"
      names="date_last_updated last_updated_by_link keywords"/>
    <fix:param required="false"
      reps="git_repository-full"
      names="code_import_link reviewer_link git_https_url
             date_last_repacked date_last_scanned default_branch
             description"/>
    <fix:param required="false"
      reps="language-full"
      names="plural_forms plural_expression"/>
    <fix:param required="false"
      reps="livefs-full"
      names="keep_binary_files_days"/>
    <fix:param required="false"
      reps="message-full question_message-full"
      names="parent_link date_deleted date_last_edited"/>
    <fix:param required="false"
      reps="message_revision-full"
      names="date_deleted"/>
    <fix:param required="false"
      reps="milestone-full project_group_milestone-full"
      names="date_targeted code_name release_link summary"/>
    <fix:param required="false"
      reps="oci_push_rule-full"
      names="username"/>
    <fix:param required="false"
      reps="package_upload-full"
      names="copy_source_archive_link changes_file_url"/>
    <fix:param required="false"
      reps="person-full"
      names="team_owner_link archive_link description
             preferred_email_address_link"/>
    <fix:param required="false"
      reps="project-full"
      names="homepage_url download_url screenshots_url wiki_url
             bug_supervisor_link bug_tracker_link driver_link
             project_group_link translation_focus_link
             translationgroup_link commercial_subscription_link
             freshmeat_project sourceforge_project remote_product
             reviewer_whiteboard license_info programming_language"/>
    <fix:param required="false"
      reps="project_group-full"
      names="homepage_url wiki_url driver_link bug_tracker_link
             translationgroup_link freshmeat_project sourceforge_project"/>
    <fix:param required="false"
      reps="project_release-full"
      names="changelog release_notes"/>
    <fix:param required="false"
      reps="project_series-full"
      names="branch_link driver_link release_finder_url_pattern"/>
    <fix:param required="false"
      reps="binary_package_publishing_history-full
            source_package_publishing_history-full"
      names="phased_update_percentage date_made_pending date_published
             date_removed date_superseded scheduled_deletion_date
             removed_by_link removal_comment copied_from_archive_link
             creator_link sponsor_link package_signer_link
             packageupload_link"/>
    <fix:param required="false"
      reps="question-full"
      names="answer_link answerer_link assignee_link date_due
             date_last_response date_solved"/>
    <fix:param required="false"
      reps="revision_status_report-full"
      names="ci_build_link distro_arch_series_link url result_summary
             date_started date_finished properties"/>
    <fix:param required="false"
      reps="snap-full"
      names="auto_build_archive_link branch_link git_repository_link
             git_repository_url git_ref_link git_path distro_series_link
             store_series_link"/>
    <fix:param required="false"
      reps="snap_base-full"
      names="features"/>
    <fix:param required="false"
      reps="source_package-full"
      names="productseries_link latest_published_component_name"/>
    <fix:param required="false"
      reps="source_package_recipe-full"
      names="base_branch_link base_git_repository_link
             daily_build_archive_link last_build_link"/>
    <fix:param required="false"
      reps="specification-full"
      names="approver_link assignee_link completer_link drafter_link
             starter_link milestone_link date_completed date_started
             specification_url whiteboard"/>
    <fix:param required="false"
      reps="team-full"
      names="description team_description preferred_email_address_link
             default_membership_period default_renewal_period
             archive_link"/>
    <fix:param required="false"
      reps="team_membership-full"
      names="date_expires date_joined last_change_comment"/>
    <fix:param required="false"
      reps="translation_import_queue_entry-full"
      names="error_output sourcepackage_link productseries_link
             distroseries_link"/>
    <fix:param required="false"
      reps="vulnerability-full"
      names="cve_link date_coordinated_release date_made_public
             date_notice_issued mitigation notes cvss metadata"/>
    <fix:param required="false"
      reps="webhook-full"
      names="git_ref_pattern"/>
    <fix:param required="false"
      reps="webhook_delivery-full"
      names="successful date_first_sent date_sent date_scheduled
             error_message"/>

    <!-- Lists of strings -->
    <fix:param repeating="true"
      reps="bug-full project-full bug_subscription_filter-full"
      names="tags licenses"/>
    <fix:param repeating="true" required="false"
      reps="git_repository-full rock_recipe_build_request-full"
      names="builder_constraints architectures"/>

    <!-- Untyped params that are actually integers -->
    <fix:param type="int"
      reps="bug-full code_review_comment-full faq-full
            git_repository-full livefs-full package_upload-full
            packageset-full preview_diff-full question-full
            translation_file-full translation_import_queue_entry-full
            translation_template-full vulnerability-full"
      names="id"/>
    <fix:param type="int"
      reps="translation_template-full"
      names="priority"/>
    <fix:param type="int"
      reps="question_message-full"
      names="index"/>
    <fix:param type="int"
      reps="message_revision-full"
      names="revision"/>

    <!-- Untyped params that are actually booleans -->
    <fix:param type="boolean"
      reps="archive-full"
      names="build_debug_symbols publish_debug_symbols
             permit_obsolete_series_uploads"/>
    <fix:param type="boolean"
      reps="builder-full"
      names="builderok"/>
    <fix:param type="boolean"
      reps="charm_recipe-full craft_recipe-full rock_recipe-full
            snap-full"
      names="use_fetch_service"/>
    <fix:param type="boolean"
      reps="distribution-full"
      names="redirect_default_traversal redirect_release_uploads
             supports_mirrors supports_ppas"/>
    <fix:param type="boolean"
      reps="distro_series-full"
      names="publish_i18n_index publish_by_hash proposed_not_automatic
             strict_supported_component_dependencies"/>
    <fix:param type="boolean"
      reps="oci_recipe_build-full"
      names="estimate"/>
    <fix:param type="boolean"
      reps="package_upload-full"
      names="contains_build contains_copy contains_source"/>
    <fix:param type="boolean"
      reps="processor-full"
      names="build_by_default restricted supports_nonvirtualized"/>
    <fix:param type="boolean"
      reps="snap-full"
      names="pro_enable"/>
    <fix:param type="boolean"
      reps="specification-full"
      names="direction_approved"/>

    <!-- Fields that are redacted for anonymous or unprivileged users -->
    <fix:param type="launchpadlib::types::MaybeRedacted&lt;bool&gt;"
      reps="project-full"
      names="project_reviewed license_approved"/>
    <fix:param type="launchpadlib::types::MaybeRedacted&lt;i32&gt;"
      reps="person-full team-full"
      names="id"/>
  </fix:params>

  <xsl:variable name="param-fixes" select="document('')/xsl:stylesheet/fix:params/fix:param"/>

  <xsl:template match="wadl:application/wadl:representation/wadl:param" priority="-0.25">
    <xsl:variable name="rep" select="concat(' ', ../@id, ' ')"/>
    <xsl:variable name="name" select="concat(' ', @name, ' ')"/>
    <xsl:variable name="fixes" select="$param-fixes[
        (@reps = '*' or contains(concat(' ', normalize-space(@reps), ' '), $rep))
        and contains(concat(' ', normalize-space(@names), ' '), $name)]"/>
    <xsl:copy>
      <xsl:apply-templates select="@*"/>
      <xsl:for-each select="$fixes/@required | $fixes/@type | $fixes/@repeating">
        <xsl:attribute name="{local-name()}">
          <xsl:value-of select="."/>
        </xsl:attribute>
      </xsl:for-each>
      <xsl:apply-templates select="node()"/>
    </xsl:copy>
  </xsl:template>

  <xsl:template match="xhtml:tbody/xhtml:tr[xhtml:th[@colspan='2']]">
        <xsl:element name="xhtml:tr">
            <xsl:value-of select="xhtml:th"/>
            <xsl:element name="xhtml:td">
                <xsl:value-of select="xhtml:td/text()"/>
                <!-- Include the text() from all the following xhtml:td elements before the next xhtml:th -->
                <xsl:apply-templates select="following-sibling::xhtml:tr[1]/xhtml:td/text()"/>
            </xsl:element>
        </xsl:element>
    </xsl:template>

    <xsl:template match="xhtml:tbody/xhtml:tr[xhtml:td[normalize-space(text()) = '\']]" />

  <xsl:template match="wadl:method/wadl:request/wadl:param[not(wadl:doc)]">
      <xsl:copy>
        <xsl:apply-templates select="@*"/>
          <xsl:apply-templates select="node()"/>
          <xsl:call-template name="import-doc">
              <xsl:with-param name="name" select="@name"/>
              <xsl:with-param name="tbody" select="../../wadl:doc/xhtml:table[@class='rst-docutils field-list' and @frame='void' and @rules='none']/xhtml:tbody"/>
          </xsl:call-template>
      </xsl:copy>
  </xsl:template>

  <xsl:template match="wadl:method/wadl:request/wadl:representation/wadl:param[not(wadl:doc)]">
      <xsl:copy>
        <xsl:apply-templates select="@*"/>
          <xsl:apply-templates select="node()"/>
          <xsl:call-template name="import-doc">
              <xsl:with-param name="name" select="@name"/>
              <xsl:with-param name="tbody" select="../../../wadl:doc/xhtml:table[@class='rst-docutils field-list' and @frame='void' and @rules='none']/xhtml:tbody"/>
          </xsl:call-template>
      </xsl:copy>

  </xsl:template>

  <xsl:template name="import-doc">
      <xsl:param name="name"/>
      <xsl:param name="tbody"/>

      <xsl:variable name="doc" select="$tbody/xhtml:tr/xhtml:th[text()=$name or text()=concat('param ', $name, ':') or text()=concat($name, ':')]/following-sibling::xhtml:td[1]"/>
      <xsl:if test="$doc">
          <xsl:element name="wadl:doc">
              <xsl:value-of select="$doc"/>
          </xsl:element>
      </xsl:if>
  </xsl:template>

  <xsl:template match="wadl:method/wadl:doc/xhtml:table[@class='rst-docutils field-list' and @frame='void' and @rules='none']"/>

  <!-- Mark all content_templates attributes everywhere as optional -->
  <xsl:template match="wadl:param[@name='content_templates']">
    <xsl:copy>
    <xsl:apply-templates select="@*"/>
      <xsl:attribute name="required">false</xsl:attribute>
    <xsl:apply-templates select="node()"/>
    </xsl:copy>
  </xsl:template>

  <!-- Add String return type to getArchiveSubscriptionURL methods -->
  <xsl:template match="wadl:method[@id='person-getArchiveSubscriptionURL' or @id='team-getArchiveSubscriptionURL']">
    <xsl:copy>
      <xsl:apply-templates select="@*"/>
      <xsl:apply-templates select="wadl:doc"/>
      <xsl:apply-templates select="wadl:request"/>
      <xsl:element name="wadl:response">
        <xsl:element name="wadl:representation">
          <xsl:attribute name="mediaType">text/plain</xsl:attribute>
          <xsl:element name="wadl:param">
            <xsl:attribute name="name">return</xsl:attribute>
            <xsl:attribute name="type">xsd:string</xsl:attribute>
            <xsl:attribute name="style">plain</xsl:attribute>
          </xsl:element>
        </xsl:element>
      </xsl:element>
    </xsl:copy>
  </xsl:template>
</xsl:stylesheet>
