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

# Remove empty doc comments
find src -type f -exec sed -i '/^\s*\/\/\/\s*$/d' {} \;

#Fix example
cat patches/Cargo.toml >> Cargo.toml

find src/ -type f -name "*.rs" -exec sed -i 's/models::models/models/g' {} +
find src/ -type f -name "*.rs" -exec sed -i -E "s/(::)?std::path::PathBuf/crate::patches::better_file_upload::File<'_>/g" {} +

rm -r src/patches
cp -r patches src/patches
printf "\npub mod patches;" >> src/lib.rs

cargo fmt
cargo build
cargo test
