<!-- XSLT Patch -->
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:wadl="http://research.sun.com/wadl/2006/10" xmlns:xhtml="http://www.w3.org/1999/xhtml">
  <!-- Identity template to copy all nodes and attributes unchanged -->
  <xsl:template match="@* | node()">
    <xsl:copy>
      <xsl:apply-templates select="@* | node()"/>
    </xsl:copy>
  </xsl:template>

  <!-- Set type of various id param to int -->
  <xsl:template match="wadl:representation[
  @id='bug-full'
  or @id='package_upload'
  or @id='package_upload-full'
      ]/wadl:param[@name='id']">
    <xsl:copy>
    <xsl:apply-templates select="@*"/>
      <xsl:attribute name="type">int</xsl:attribute>
    <xsl:apply-templates select="node()"/>
    </xsl:copy>
  </xsl:template>

  <!-- Mark tags param as repeating in bug-full -->
  <xsl:template match="wadl:representation[@id='bug-full']/wadl:param[@name='tags']">
    <xsl:copy>
      <xsl:apply-templates select="@*"/>
      <xsl:attribute name="repeating">true</xsl:attribute>
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

  <!-- Mark various attributes in distribution-full optional -->
  <xsl:template match="wadl:representation[@id='distribution-full' or @id='distribution']/wadl:param[
  @name='oci_project_admin_link'
  or @name='commercial_subscription_link'
      ]">
    <xsl:copy>
    <xsl:apply-templates select="@*"/>
      <xsl:attribute name="required">false</xsl:attribute>
    <xsl:apply-templates select="node()"/>
    </xsl:copy>
  </xsl:template>

  <!-- Mark various attributes in distro_series optional -->
  <xsl:template match="wadl:representation[@id='distro_series' or @id='distro_series-full']/wadl:param[
  @name='datereleased'
  or @name='driver_link'
  or @name='parent_series_link'
      ]">
    <xsl:copy>
    <xsl:apply-templates select="@*"/>
      <xsl:attribute name="required">false</xsl:attribute>
    <xsl:apply-templates select="node()"/>
    </xsl:copy>
  </xsl:template>

  <!-- Mark all content_templates attributes everywhere as optional -->
  <xsl:template match="wadl:param[@name='content_templates']">
    <xsl:copy>
    <xsl:apply-templates select="@*"/>
      <xsl:attribute name="required">false</xsl:attribute>
    <xsl:apply-templates select="node()"/>
    </xsl:copy>
  </xsl:template>

  <xsl:template match="wadl:representation[@id='package_upload' or @id='package_upload-full']/wadl:param[
  @name='copy_source_archive_link'
      ]">
    <xsl:copy>
    <xsl:apply-templates select="@*"/>
      <xsl:attribute name="required">false</xsl:attribute>
    <xsl:apply-templates select="node()"/>
    </xsl:copy>
  </xsl:template>

  <!-- Retype various attributes in distribution-full as boolean -->
  <xsl:template match="wadl:representation[@id='distribution-full' or @id='distribution']/wadl:param[
  @name='redirect_default_traversal'
  or @name='redirect_release_uploads'
  or @name='supports_mirrors'
  or @name='supports_ppas'
      ]">
    <xsl:copy>
    <xsl:apply-templates select="@*"/>
      <xsl:attribute name="type">boolean</xsl:attribute>
    <xsl:apply-templates select="node()"/>
    </xsl:copy>
  </xsl:template>

  <!-- Retype various attributes in distro_series as booleans -->
  <xsl:template match="wadl:representation[@id='distro_series' or @id='distro_series-full']/wadl:param[
  @name='publish_i18n_index'
  or @name='publish_by_hash'
  or @name='proposed_not_automatic'
      ]">
    <xsl:copy>
    <xsl:apply-templates select="@*"/>
      <xsl:attribute name="type">boolean</xsl:attribute>
    <xsl:apply-templates select="node()"/>
    </xsl:copy>
  </xsl:template>

  <!-- Mark bug reporting guidelines as optional -->
  <xsl:template match="wadl:representation[@name='bug_reporting_guidelines' or @name='bug_tracker_link' or @name='date_next_suggest_packaging']">
    <xsl:copy>
    <xsl:apply-templates select="@*"/>
      <xsl:attribute name="required">false</xsl:attribute>
    <xsl:apply-templates select="node()"/>
    </xsl:copy>
  </xsl:template>

</xsl:stylesheet>
