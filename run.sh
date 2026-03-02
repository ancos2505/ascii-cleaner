#!/bin/bash

# set -ex

./build.sh

pushd ./dist/
./ascii-cleaner detect myfile.txt
./ascii-cleaner detect myfile.txt | jq .
popd
