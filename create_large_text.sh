#!/usr/bin/env bash

# Create a large text file from fixtures/alice.txt
# Simply concatenate the file N times

# Usage: ./create_large_text.sh

for _ in {1..1000}; do cat fixtures/alice.txt >> fixtures/large.txt; done
