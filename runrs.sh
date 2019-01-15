#!/bin/bash

rm rs/target/release/rs
rm out.wmv

pushd rs
cargo build --release
popd

./rs/target/release/rs romance.mp3 out.wmv

xdg-open out.wmv