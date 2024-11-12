#!/usr/bin/env bash

# Generate Client
rm src/apis src/models docs -rf

./node_modules/\@openapitools/openapi-generator-cli/main.js generate \
-g rust \
--additional-properties=packageName=vrchatapi,supportAsync=true,avoidBoxedModels=true \
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
sed -i 's/, features = \["json", "multipart"\]/, default-features = false, features = \["json", "cookies", "multipart"\]/g' Cargo.toml

#Fix example
printf "\n[dev-dependencies]\ntokio = { version = '1', features = ['macros', 'rt-multi-thread'] }" >> Cargo.toml

#Add feature section to Cargo.toml
printf "\n[features]\ndefault = [\"reqwest/default\"]" >> Cargo.toml


# https://github.com/vrchatapi/specification/issues/241
cat patches/2FA_Current_User.rs >> src/models/current_user.rs
sed -i 's/pub use self::current_user::CurrentUser;/pub use self::current_user::{EitherUserOrTwoFactor, CurrentUser};/g' src/models/mod.rs
sed -i 's/Result<models::CurrentUser, Error<GetCurrentUserError>>/Result<models::EitherUserOrTwoFactor, Error<GetCurrentUserError>>/g' src/apis/authentication_api.rs

# https://github.com/vrchatapi/vrchatapi-rust/pull/29
sed -i "s/local_var_req_builder = local_var_req_builder.json(&\(.*\));/if let Some(\1) = \1 { \0 }/g" src/apis/files_api.rs

#https://github.com/vrchatapi/vrchatapi-rust/pull/30
perl -0pi -e 's|(fn\s+[^(]*\([^)]*)file:\s+:?:?std::path::PathBuf,?([^)]*)((?:(?!\/\/ TODO: support file upload for '\''file'\'' parameter)[\s\S])*)\/\/ TODO: support file upload for '\''file'\'' parameter|\1file: impl Into<::std::borrow::Cow<'\''static, [u8]>>,\n\tfilename: impl Into<::std::borrow::Cow<'\''static, str>>,\n\tmime_type: &str,\2\3let part = reqwest::multipart::Part::bytes(file).file_name(filename).mime_str(mime_type)?;\n\tlocal_var_form = local_var_form.part("file", part);|g' src/apis/files_api.rs

cargo fmt
cargo build
cargo test
