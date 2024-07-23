pub use vrchatapi::apis;

#[tokio::main]
async fn main() {
    let config = apis::configuration::Configuration {
        // credentials do not belong in source code, use arguments or environment variables
        basic_auth: Some((String::from("username"), Some(String::from("password")))),
        ..Default::default()
    };

    let current_user = match apis::authentication_api::get_current_user(&config)
        .await
        .expect("Failed to get current user")
    {
        vrchatapi::models::GetCurrentUser200Response::CurrentUser(user) => user,
        vrchatapi::models::GetCurrentUser200Response::TwoFactorRequired(_two_factor_resp) => {
            // The API returns you a list of methods to verify your 2FA, this determines what function to call next
            // this example assumes you have a TOTP code

            let code = "123456"; // grab this via stdin or generate it

            let resp = apis::authentication_api::verify2_fa(
                &config,
                vrchatapi::models::TwoFactorAuthCode {
                    code: code.to_string(),
                },
            )
            .await
            .expect("Failed to get 2FA response");

            if !resp.verified {
                panic!("Failed to verify 2FA");
            }

            match apis::authentication_api::get_current_user(&config)
                .await
                .expect("Failed to get current user")
            {
                vrchatapi::models::GetCurrentUser200Response::CurrentUser(user) => user,
                _ => panic!("Got 2FA response, even after verifying 2FA"),
            }
        }
    };

    println!("Current User: {:?}", current_user.display_name);

    let online = apis::system_api::get_current_online_users(&config)
        .await
        .expect("Failed to get online users");
    println!("Current Online Users: {}", online);
}
