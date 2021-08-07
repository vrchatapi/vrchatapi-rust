#!/bin/bash

# Generate Client
rm src/apis src/models docs -rf

openapi-generator-cli generate \
-g rust \
--additional-properties=packageName=vrchatapi,supportAsync=false \
--git-user-id=vrchatapi \
--git-repo-id=vrchatapi-rust \
-o . \
-i ../specification/openapi.yaml \
--http-user-agent="vrchatapi-rust"

rm docs -rf

#description = "VRChat API Library for Rust"
#license = "MIT"

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
