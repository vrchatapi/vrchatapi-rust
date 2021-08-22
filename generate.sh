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

sed -i '/^edition = "2018"/i license = "MIT"' Cargo.toml
sed -i '/^edition = "2018"/a description="VRChat API Library for Rust"' Cargo.toml

# Remove unwanted lines from README
#sed -i '/Uncomment the following line to set a prefix/d' ./README.md
#sed -i '/apiKeyPrefix/d' ./README.md
#sed -i '/Uncomment the following line to set a prefix/d' ./docs/*.md
#sed -i '/apiKeyPrefix/d' ./docs/*.md
#sed -i 's/api.deleteUserById/api.getUser/g' ./README.md
#
#sed -i 's/@default false/@default true/g' ./src/ApiClient.js
#sed -i 's/enableCookies = false/enableCookies = true/g' ./src/ApiClient.js

#npm run build
