#!/usr/bin/env bash
set -euo pipefail

if [ ${#} -le 0 ]
then
  echo "Usage: generate.sh <openapi.json>" >&2
  exit 1
fi

# Generate Client
rm src/apis src/models docs -rf

./node_modules/\@openapitools/openapi-generator-cli/main.js generate \
-g rust \
-t ./templates \
'--additional-properties=packageName=vrchatapi,supportAsync=true,avoidBoxedModels=true,library=reqwest,reqwestDefaultFeatures=reqwest/cookies,supportMiddleware=true,repositoryUrl="https://github.com/vrchatapi/vrchatapi-rust",description="VRChat API Client for Rust",infoEmail="vrchatapi.lpv0t@aries.fyi"' \
--git-user-id=vrchatapi \
--git-repo-id=vrchatapi-rust \
-o . \
-i "${1}" \
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

#Fix example
printf "\n[dev-dependencies]\ntokio = { version = '1', features = ['macros', 'rt-multi-thread'] }" >> Cargo.toml

find src/ -type f -name "*.rs" -exec sed -i 's/models::models/models/g' {} +

cargo fmt
cargo build
cargo test
