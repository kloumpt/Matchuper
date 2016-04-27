#!/bin/bash
cd fonts && find -type f -name "*.woff" | awk '{gsub("./", "", $1); name=$1; gsub(".woff", "", name); print "@font-face {"; print "   font-family: " name ";"; print "   src: url(/fonts/" $1 ");"; print "}"; print "";}'
