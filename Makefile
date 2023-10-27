all: wadl/1.0.wadl wadl/devel.wadl wadl/beta.wadl

wadl/%.wadl:
	curl -H "Accept: application/vd.sun.wadl+xml" https://api.launchpad.net/$*/ -o $@
