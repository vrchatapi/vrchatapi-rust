use std::io::{self, Write};
use reqwest::header::{HeaderMap, HeaderValue};
pub use vrchatapi::apis;
use vrchatapi::models::{EitherUserOrTwoFactor, TwoFactorAuthCode, TwoFactorEmailCode};

#[tokio::main]
async fn main() {
    let mut config = apis::configuration::Configuration::default();
    config.basic_auth = Some((String::from("username"), Some(String::from("password"))));
    config.user_agent = Some(String::from("ProjectName/0.0.1 email@example.com"));

    let mut header_map = HeaderMap::new();
    header_map.append("Cookie", HeaderValue::from_str(
        &"auth=[AUTH_COOKIE_HERE]; twoFactorAuth=[TWO_FACTOR_AUTH_COOKIE_HERE]"
    ).unwrap());

    config.client = reqwest::Client::builder()
            .cookie_store(true)
            .default_headers(header_map)
            .build()
            .unwrap();

    let user = apis::authentication_api::get_current_user(&config)
        .await
        .unwrap();

    match user {
        EitherUserOrTwoFactor::CurrentUser(user) => println!("Current user: {}", user.display_name),
        EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => println!("cookie invalid")
    }
}
