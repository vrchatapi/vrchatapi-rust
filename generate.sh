#!/bin/bash

# Generate Client
rm src/apis src/models docs -rf

java -jar ./openapi-generator-cli.jar generate \
-g rust \
--additional-properties=packageName=vrchatapi,supportAsync=false \
--git-user-id=vrchatapi \
--git-repo-id=vrchatapi-rust \
-o . \
-i https://raw.githubusercontent.com/vrchatapi/specification/gh-pages/openapi.yaml \
--http-user-agent="vrchatapi-rust"
#--global-property debugOperations=true

# Add license and description to Cargo.toml
sed -i '/^edition = "2018"/i license = "MIT"' Cargo.toml
sed -i '/^edition = "2018"/a description="VRChat API Library for Rust"' Cargo.toml

# Remove messily pasted markdown at top of every file
find src -type f -exec sed -i '/VRChat API Banner/d' {} \;
# Remove openapi version in every file
find src -type f -exec sed -i '/The version of the OpenAPI document/d' {} \;

# Cookie storage
sed -i 's/Client::new()/Client::builder().cookie_store(true).build().unwrap()/g' src/apis/configuration.rs

cargo build
