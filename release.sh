#!/usr/bin/env sh

# Build the website and add the CNAME file
trunk build --release -d docs &>/dev/null &&
	printf "resume.etiennecollin.com" >docs/CNAME &&
	echo "Website built and CNAME file created" ||
	{ echo "Error building the website"; exit 1; }

git add docs &>/dev/null &&
	git commit -m "Released website" docs &>/dev/null &&
	echo "Website commited to git" ||
    { echo "Error commiting website to git"; exit 1; }
