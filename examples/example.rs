use std::io::{self, Write};
pub use vrchatapi::apis;
use vrchatapi::models::{EitherUserOrTwoFactor, TwoFactorAuthCode, TwoFactorEmailCode};

#[tokio::main]
async fn main() {
    let mut config = apis::configuration::Configuration::default();
    config.basic_auth = Some((String::from("username"), Some(String::from("password"))));
    config.user_agent = Some(String::from("ProjectName/0.0.1 email@example.com"));
    match apis::authentication_api::get_current_user(&config)
        .await
        .unwrap()
    {
        vrchatapi::models::EitherUserOrTwoFactor::CurrentUser(me) => {
            println!("Username: {}", me.username.unwrap())
        }
        vrchatapi::models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth) => {
            if requires_auth
                .requires_two_factor_auth
                .contains(&String::from("emailOtp"))
            {
                let code = read_user_input("Please enter your Email 2fa code: ");
                if let Err(err) = apis::authentication_api::verify2_fa_email_code(
                    &config,
                    TwoFactorEmailCode::new(code),
                )
                .await
                {
                    eprintln!("Error verifying 2FA email code: {}", err);
                }
            } else {
                let code = read_user_input("Please enter your Authenticator 2fa code: ");
                if let Err(err) =
                    apis::authentication_api::verify2_fa(&config, TwoFactorAuthCode::new(code))
                        .await
                {
                    eprintln!("Error verifying 2FA auth code: {}", err);
                }
            }
        }
    }

    let user = apis::authentication_api::get_current_user(&config)
        .await
        .unwrap();

    match user {
        EitherUserOrTwoFactor::CurrentUser(user) => println!("Current user: {}", user.display_name),
        EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => println!("cookie invalid"),
    }
}

fn read_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
