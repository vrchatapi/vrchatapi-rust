pub use vrchatapi::apis;

fn main() {
    let config = apis::configuration::Configuration::default();

    let online = apis::system_api::get_current_online_users(&config).unwrap();

    println!("Current Online Users: {}", online);
}