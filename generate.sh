#!/bin/bash

# Generate Client
rm src/apis src/models docs -rf

openapi-generator-cli generate \
-g rust \
--additional-properties=packageName=vrchatapi,packageVersion=1.0.4,supportAsync=false \
--git-user-id=vrchatapi \
--git-repo-id=vrchatapi-rust \
-o . \
-i https://vrchatapi.github.io/specification/openapi.yaml \
--http-user-agent="vrchatapi-rust"

# Add license and description to Cargo.toml
sed -i '/^edition = "2018"/i license = "MIT"' Cargo.toml
sed -i '/^edition = "2018"/a description="VRChat API Library for Rust"' Cargo.toml

# Remove messily pasted markdown at top of every file
find src -type f -exec sed -i '/VRChat API Banner/d' {} \;

cargo build
