#!/bin/bash

set -x

MUSL_TARGET="$(arch)-unknown-linux-musl"

cargo build --release --target=${MUSL_TARGET}

mkdir -p dist

cp -v ./target/${MUSL_TARGET}/release/ascii-cleaner ./dist/
cp -v ./myfile.txt ./dist/
pushd ./dist/
./ascii-cleaner 
./ascii-cleaner detect myfile.txt
popd
