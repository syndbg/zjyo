#!/bin/bash

echo "Testing Rust z implementation"
echo "==============================="

# Test adding directories
echo "Adding test directories..."
cd /test-dirs/project && z --add
cd /test-dirs/documents/reports && z --add
cd /test-dirs/downloads/tools && z --add
cd /test-dirs/project/src && z --add

echo -e "\nListing all tracked directories:"
z -l

echo -e "\nTesting pattern matching:"
echo "z proj (should match project directories):"
z -e proj

echo -e "\nz doc (should match documents):"
z -e doc

echo -e "\nz tool (should match tools):"
z -e tool

echo -e "\nTesting rank-based search:"
z -r -e proj

echo -e "\nTesting time-based search:"
z -t -e proj

echo -e "\nTesting current directory restriction:"
cd /test-dirs
z -c -e proj

echo -e "\nRemoving a directory:"
z -x
z -l

echo -e "\nTest completed!"