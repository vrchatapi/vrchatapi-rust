#!/usr/bin/env bash
set -euo pipefail

if [ ${#} -le 0 ]
then
  echo "Usage: generate.sh <openapi.json>" >&2
  exit 1
fi

# Generate Client
rm src/apis src/models docs -rf

openapi-generator generate \
-g rust \
-t ./templates \
'--additional-properties=packageName=vrchatapi,supportAsync=true,avoidBoxedModels=true,library=reqwest,supportMiddleware=true,repositoryUrl=https://github.com/vrchatapi/vrchatapi-rust,description="VRChat API Client for Rust",infoEmail="vrchatapi.lpv0t@aries.fyi"' \
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

# Fix example
# - tokio is needed as an async runtime
printf "\n[dev-dependencies]\ntokio = { version = '1', features = ['macros', 'rt-multi-thread'] }" >> Cargo.toml

find src/ -type f -name "*.rs" -exec sed -i 's/models::models/models/g' {} +
find src/ -type f -name "*.rs" -exec sed -i -E "s/(::)?std::path::PathBuf/crate::patches::better_file_upload::File<'_>/g" {} +

# Fix infinite size of Transaction Agreement
sed -i 's/TransactionAgreement(models::TransactionAgreement),/TransactionAgreement(Box<models::TransactionAgreement>),/' src/models/transaction_agreement.rs

rm -r src/patches
cp -r patches src/patches
printf "\npub mod patches;" >> src/lib.rs

cargo fmt
