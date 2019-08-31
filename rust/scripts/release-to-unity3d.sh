#!/usr/bin/env bash

cargo build --release

source=target/release/librustlib.so

cp -v $source ../Assets/Plugins/lib
cp -v $source ../build/01_Data/Plugins/librustlib.so
