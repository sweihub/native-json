#!/bin/bash
# update markdown to module doc

MD=$1
RS=$2
TEMP=markdown-module-doc.rs

if [ -z $RS ]; then
    echo "Usage: docs.sh markdown.md rust.js"
    exit
fi

# comment the markdown 
while IFS= read i; do
    echo "//!$i"
done < $1 > $TEMP

# strip the existing module doc
cat $2 | grep -v "^//!" >> $TEMP

mv $TEMP $RS

