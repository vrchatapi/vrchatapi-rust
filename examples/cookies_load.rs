use reqwest::cookie::CookieStore;
use reqwest::header::HeaderValue;
use std::str::FromStr;
use std::sync::Arc;
pub use vrchatapi::apis;
use vrchatapi::models::EitherUserOrTwoFactor;

#[tokio::main]
async fn main() {
    let mut config = apis::configuration::Configuration::default();
    config.basic_auth = Some((String::from("username"), Some(String::from("password"))));
    config.user_agent = Some(String::from("ProjectName/0.0.1 email@example.com"));

    let mut jar = reqwest::cookie::Jar::default();
    jar.set_cookies(
        &mut [HeaderValue::from_str(
            &"auth=[AUTH_COOKIE_HERE], twoFactorAuth=[TWO_FACTOR_AUTH_COOKIE_HERE]",
        )
        .expect("Cookie not okay")]
        .iter(),
        &url::Url::from_str("https://api.vrchat.cloud").expect("Url not okay"),
    );
    let jar = Arc::new(jar);

    config.client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(jar)
        .build()
        .unwrap();

    let user = apis::authentication_api::get_current_user(&config)
        .await
        .unwrap();

    match user {
        EitherUserOrTwoFactor::CurrentUser(user) => println!("Current user: {}", user.display_name),
        EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => println!("cookie invalid"),
    }
}
