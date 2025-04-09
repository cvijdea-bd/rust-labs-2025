#!/usr/bin/sh

java_in=$1
class_out=${1%.java}.class
out=out/lab1

rm -rf lab1/java/$class_out
rm -rf $out

echo "Converting $java_in to DEX format..."
javac $1
if [ $? -ne 0 ]; then
    echo "Compilation failed."
    exit 1
fi
echo "Compiling to DEX format..."
mkdir $out -p
d8 --output $out $class_out
if [ $? -ne 0 ]; then
    echo "Conversion to DEX format failed."
    exit 1
fi
echo "Conversion to DEX format completed successfully."
dex2smali $out/classes.dex -o $out --force