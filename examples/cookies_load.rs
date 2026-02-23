use reqwest::cookie::CookieStore;
use reqwest::header::HeaderValue;
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut config = ::vrchatapi::apis::configuration::Configuration::default();
    config.basic_auth = Some((String::from("username"), Some(String::from("password"))));
    config.user_agent = Some(String::from("ExampleProgram/0.0.1 my@email.com"));

    let jar = reqwest::cookie::Jar::default();
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
        .cookie_provider(jar)
        .build()
        .unwrap()
        .into();

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
}
