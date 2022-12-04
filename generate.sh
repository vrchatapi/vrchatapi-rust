#!/bin/bash

# Generate Client
rm src/apis src/models docs -rf

./node_modules/\@openapitools/openapi-generator-cli/main.js generate \
-g rust \
--additional-properties=packageName=vrchatapi,supportAsync=false \
--git-user-id=vrchatapi \
--git-repo-id=vrchatapi-rust \
-o . \
-i ../specification/dist/openapi.yaml \
--http-user-agent="vrchatapi-rust"
#--global-property debugOperations=true

# Update entire description (replace entire line, match the random data there) line in Cargo.toml
sed -i 's/^description = ".*"/description = "VRChat API Client for Rust"/' Cargo.toml

# Remove messily pasted markdown at top of every file
find src -type f -exec sed -i '/VRChat API Banner/d' {} \;
# Remove openapi version in every file
find src -type f -exec sed -i '/The version of the OpenAPI document/d' {} \;

# Cookie storage
sed -i 's/Client::new()/Client::builder().cookie_store(true).build().unwrap()/g' src/apis/configuration.rs

# https://github.com/OpenAPITools/openapi-generator/issues/14171
# Replace Option<SortOption with Option<crate::models::SortOption in src/apis
sed -i 's/Option<SortOption>/Option<crate::models::SortOption>/g' src/apis/*.rs
# Replace Option<ReleaseStatus with Option<crate::models::ReleaseStatus in src/apis
sed -i 's/Option<ReleaseStatus>/Option<crate::models::ReleaseStatus>/g' src/apis/*.rs
# Replace Option<OrderOption with Option<crate::models::OrderOption in src/apis
sed -i 's/Option<OrderOption>/Option<crate::models::OrderOption>/g' src/apis/*.rs
# Replace message_type: InviteMessageType with message_type: crate::models::InviteMessageType in src/apis
sed -i 's/message_type: InviteMessageType/message_type: crate::models::InviteMessageType/g' src/apis/*.rs

# -Missing Github Issue-
# Append patches/InviteMessageType-Display.rs to invite_message_type.rs
cat patches/InviteMessageType-Display.rs >> src/models/invite_message_type.rs
# Remove the ToString section and it's included function
# impl ToString for InviteMessageType {
#     fn to_string(&self) -> String {
#         match self {
#             Self::Message => String::from("message"),
#             Self::Response => String::from("response"),
#             Self::Request => String::from("request"),
#             Self::RequestResponse => String::from("requestResponse"),
#         }
#     }
# }
sed -z -i 's/impl ToString for InviteMessageType {\n[ a-zA-Z_\(\)&-\>{\n:=",]*}\n    }\n}//g' src/models/invite_message_type.rs


cargo build
