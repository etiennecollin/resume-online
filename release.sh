#!/usr/bin/env bash

# Build the website and add the CNAME file
trunk build --release -d docs &>/dev/null &&
	printf "resume.etiennecollin.com" >docs/CNAME &&
	echo "Website built and CNAME file added" ||
	echo "Error building the website"
