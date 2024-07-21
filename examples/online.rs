pub use vrchatapi::apis;

#[tokio::main]
async fn main() {
    let mut config = apis::configuration::Configuration::default();
    config.basic_auth = Some((String::from("username"), Some(String::from("password"))));

    let me = apis::authentication_api::get_current_user(&config).await.unwrap();
    println!("Username: {}", me.username.unwrap());

    let online = apis::system_api::get_current_online_users(&config).await.unwrap();
    println!("Current Online Users: {}", online);
}