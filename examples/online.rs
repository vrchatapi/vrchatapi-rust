pub use vrchatapi::apis;

fn main() {
    let mut config = apis::configuration::Configuration::default();
    config.basic_auth = Some((String::from("username"), Some(String::from("password"))));

    match apis::authentication_api::get_current_user(&config).unwrap() {
        vrchatapi::models::EitherUserOrTwoFactor::CurrentUser(me) => println!("Username: {}", me.username.unwrap()),
        vrchatapi::models::EitherUserOrTwoFactor::RequiresTwoFactorAuth(requires_auth) => println!("The Username requires Auth: {:?}", requires_auth.requires_two_factor_auth)
    }

    let online = apis::system_api::get_current_online_users(&config).unwrap();
    println!("Current Online Users: {}", online);
}