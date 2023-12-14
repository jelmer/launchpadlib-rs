<!-- XSLT Patch -->
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:wadl="http://research.sun.com/wadl/2006/10">
  <!-- Identity template to copy all nodes and attributes unchanged -->
  <xsl:template match="@* | node()">
    <xsl:copy>
      <xsl:apply-templates select="@* | node()"/>
    </xsl:copy>
  </xsl:template>

  <!-- Mark the team_owner_link param as optional -->
  <xsl:template match="wadl:representation[@id='person-full']/wadl:param[@name='team_owner_link']">
    <xsl:copy>
    <xsl:apply-templates select="@*"/>
      <xsl:attribute name="required">false</xsl:attribute>
    <xsl:apply-templates select="node()"/>
    </xsl:copy>
  </xsl:template>

  <!-- Set type of bug id param to int -->
  <xsl:template match="wadl:representation[@id='bug-full']/wadl:param[@name='id']">
    <xsl:copy>
    <xsl:apply-templates select="@*"/>
      <xsl:attribute name="type">int</xsl:attribute>
    <xsl:apply-templates select="node()"/>
    </xsl:copy>
  </xsl:template>

  <!-- Mark duplicate_of_link, who_made_private_link, latest_patch_uploaded and date_made_private params as optional -->
  <xsl:template match="wadl:representation[@id='bug-full']/wadl:param[@name='duplicate_of_link' or @name='latest_patch_uploaded' or @name='who_made_private_link' or @name='date_made_private']">
    <xsl:copy>
      <xsl:apply-templates select="@*"/>
      <xsl:attribute name="required">false</xsl:attribute>
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

</xsl:stylesheet>
