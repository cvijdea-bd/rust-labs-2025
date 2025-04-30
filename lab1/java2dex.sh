#!/usr/bin/sh

# Directory containing .java files (default to current dir if not specified)
src_dir=${1:-.}
out_dir=out
class_dir=$out_dir/classes

# Clean up previous outputs
rm -rf "$class_dir" "$out_dir/classes.dex" "$out_dir/smali"

# Create class output directory
mkdir -p "$class_dir"

# Compile all Java files recursively into class_dir
echo "Compiling Java files in $src_dir..."
find "$src_dir" -name "*.java" > sources.txt
javac -d "$class_dir" @sources.txt
rm sources.txt

if [ $? -ne 0 ]; then
    echo "Compilation failed."
    exit 1
fi

# Find all .class files and compile them to DEX
echo "Converting to DEX format..."
find "$class_dir" -name "*.class" > classlist.txt
d8 --output "$out_dir" @classlist.txt
rm classlist.txt

if [ $? -ne 0 ]; then
    echo "Conversion to DEX format failed."
    exit 1
fi

echo "Conversion to DEX format completed successfully."

# Convert to smali
dex2smali "$out_dir/classes.dex" -o "$out_dir/smali" --force
