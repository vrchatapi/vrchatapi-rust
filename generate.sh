#!/usr/bin/env bash

# Generate Client
rm src/apis src/models docs -rf

./node_modules/\@openapitools/openapi-generator-cli/main.js generate \
-g rust \
--additional-properties=packageName=vrchatapi,supportAsync=true \
--git-user-id=vrchatapi \
--git-repo-id=vrchatapi-rust \
-o . \
-i https://raw.githubusercontent.com/vrchatapi/specification/gh-pages/openapi.yaml \
--http-user-agent="vrchatapi-rust"
#--global-property debugOperations=true

# Update entire description (replace entire line, match the random data there) line in Cargo.toml
sed -i 's/^description = ".*"/description = "VRChat API Client for Rust"/' Cargo.toml

# Remove messily pasted markdown at top of every file
find src -type f -exec sed -i '/VRChat API Banner/d' {} \;
# Remove openapi version in every file
find src -type f -exec sed -i '/The version of the OpenAPI document/d' {} \;
# Remove empty doc comments
find src -type f -exec sed -i '/^\s*\/\/\/\s*$/d' {} \;

# Cookie storage
sed -i 's/Client::new()/Client::builder().cookie_store(true).build().unwrap()/g' src/apis/configuration.rs
sed -i 's/features = \["json", "multipart"\]/features = \["json", "cookies", "multipart"\]/g' Cargo.toml

#Fix example
printf "\n[dev-dependencies]\ntokio = { version = '1', features = ['macros', 'rt-multi-thread'] }" >> Cargo.toml

cargo build
cargo test
