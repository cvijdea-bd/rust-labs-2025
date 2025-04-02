#!/usr/bin/sh

file=$1
if [ -z "$file" ]; then
    echo "Usage: $0 <file>"
    exit 1
fi

stem=$(basename "$file" .c)

echo "Compiling $stem"
gcc -fno-stack-protector -g -o /tmp/$stem $file

echo "Running $stem"
/tmp/$stem