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
</xsl:stylesheet>
