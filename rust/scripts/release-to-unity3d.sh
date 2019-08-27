#!/usr/bin/env bash

cargo build --release

cp -v target/release/librustlib.so ../Assets/lib

