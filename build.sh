#!/bin/bash
export $(cat .env | xargs)
cargo build --release
mv target/release/clearch clearch
