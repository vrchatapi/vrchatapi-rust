use reqwest::cookie::CookieStore;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let mut config = ::vrchatapi::apis::configuration::Configuration::default();
    config.basic_auth = Some((String::from("username"), Some(String::from("password"))));
    config.user_agent = Some(String::from("ExampleProgram/0.0.1 my@email.com"));

    let cookie_store = std::sync::Arc::new(reqwest::cookie::Jar::default());
    config.client = reqwest::Client::builder()
        .cookie_provider(cookie_store.clone())
        .build()
        .unwrap();

    match ::vrchatapi::apis::authentication_api::get_current_user(&config)
        .await
        .unwrap()
    {
        ::vrchatapi::models::RegisterUserAccount200Response::CurrentUser(me) => {
            println!("Username: {}", me.username.unwrap())
        }
        ::vrchatapi::models::RegisterUserAccount200Response::RequiresTwoFactorAuth(
            requires_auth,
        ) => {
            if requires_auth
                .requires_two_factor_auth
                .contains(&::vrchatapi::models::TwoFactorAuthType::EmailOtp)
            {
                let code = read_user_input("Please enter your Email 2fa code: ");
                if let Err(err) = ::vrchatapi::apis::authentication_api::verify2_fa_email_code(
                    &config,
                    ::vrchatapi::models::TwoFactorEmailCode::new(code),
                )
                .await
                {
                    eprintln!("Error verifying 2FA email code: {}", err);
                }
            } else {
                let code = read_user_input("Please enter your Authenticator 2fa code: ");
                if let Err(err) = ::vrchatapi::apis::authentication_api::verify2_fa(
                    &config,
                    ::vrchatapi::models::TwoFactorAuthCode::new(code),
                )
                .await
                {
                    eprintln!("Error verifying 2FA auth code: {}", err);
                }
            }
        }
    }

    let user = ::vrchatapi::apis::authentication_api::get_current_user(&config)
        .await
        .unwrap();

    match user {
        ::vrchatapi::models::RegisterUserAccount200Response::CurrentUser(user) => {
            println!("Current user: {}", user.display_name)
        }
        ::vrchatapi::models::RegisterUserAccount200Response::RequiresTwoFactorAuth(_) => {
            println!("cookie invalid")
        }
    }

    println!(
        "Cookie:{}",
        cookie_store
            .cookies(&url::Url::from_str("https://api.vrchat.cloud").expect("Url not okay"))
            .expect("Cookies not found")
            .to_str()
            .expect("Cookies not valid string")
    );
}

fn read_user_input(prompt: &str) -> String {
    use ::std::io::Write;
    print!("{}", prompt);
    ::std::io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    ::std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
