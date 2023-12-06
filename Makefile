all: wadl/1.0.wadl wadl/devel.wadl wadl/beta.wadl

wadl/%.orig.wadl:
	curl -H "Accept: application/vd.sun.wadl+xml" https://api.launchpad.net/$*/ -o $@

wadl/%.wadl: wadl/%.orig.wadl fixup.xsl
	xsltproc fixup.xsl $< > $@
