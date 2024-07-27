pub use vrchatapi::apis;
use vrchatapi::models::TwoFactorEmailCode;

#[tokio::main]
async fn main() {
    let config = apis::configuration::Configuration {
        // credentials do not belong in source code, use arguments or environment variables
        basic_auth: Some((String::from("username"), Some(String::from("password")))),
        ..Default::default()
    };

    match apis::authentication_api::get_current_user(&config)
        .await
        .expect("Failed to get current user")
    {
        vrchatapi::models::GetCurrentUser200Response::CurrentUser(_user) => {
            println!("Already logged in");
        }
        vrchatapi::models::GetCurrentUser200Response::TwoFactorRequired(two_factor_resp) => {
            // The API returns you a list of methods to verify your 2FA, this determines what function to call next
            // There are only 2 ways the array of available methods looks, either ['emailOpt'] or ['otp', 'totp']
            // One deals with email verification, the other two with either TOTP or a recovery code generated from the VRChat website

            let code = "123456".to_owned(); // grab this via stdin / generate it / read it from email server

            let verified: bool;
            if two_factor_resp
                .requires_two_factor_auth
                .contains(&vrchatapi::models::two_factor_required::RequiresTwoFactorAuth::EmailOtp)
            {
                let resp = apis::authentication_api::verify2_fa_email_code(
                    &config,
                    TwoFactorEmailCode::new(code.clone()),
                )
                .await
                .expect("Failed toemail 2FA response");

                verified = resp.verified;
            } else {
                let resp = apis::authentication_api::verify2_fa(
                    &config,
                    vrchatapi::models::TwoFactorAuthCode { code },
                )
                .await
                .expect("Failed to get totp 2FA response");

                // If you have a recovery code, use this method instead
                // let resp = apis::authentication_api::verify_recovery_code(
                //     &config,
                //     vrchatapi::models::two_factor_auth_code::TwoFactorAuthCode { code },
                // )
                // .await
                // .expect("Failed to get recovery code 2FA response");

                verified = resp.verified;
            }

            if !verified {
                panic!("Failed to verify 2FA");
            }
        }
    };

    let current_user = match apis::authentication_api::get_current_user(&config)
        .await
        .expect("Failed to get current user")
    {
        vrchatapi::models::GetCurrentUser200Response::CurrentUser(user) => user,
        _ => panic!("Got 2FA response, even after verifying 2FA"),
    };

    println!("Current User: {:?}", current_user.display_name);

    let online = apis::system_api::get_current_online_users(&config)
        .await
        .expect("Failed to get online users");
    println!("Current Online Users: {}", online);
}
