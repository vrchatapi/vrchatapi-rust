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
'--additional-properties=packageName=vrchatapi,supportAsync=true,avoidBoxedModels=true,library=reqwest,supportMiddleware=true,repositoryUrl=https://github.com/vrchatapi/vrchatapi-rust,infoEmail=vrchatapi.lpv0t@aries.fyi' \
--type-mappings="file=crate::patches::better_file_upload::File<'_>" \
--inline-schema-name-mappings=Transaction_agreement=TransactionAgreementOneOf \
--git-user-id=vrchatapi \
--git-repo-id=vrchatapi-rust \
-o . \
-i "${1}" \
--http-user-agent="vrchatapi-rust"

cargo fmt
