use base32::Alphabet;
use vrchatapi::models::{EitherUserOrTwoFactor, TwoFactorAuthCode};

#[derive(Debug, thiserror::Error)]
enum Error{
    #[error("Dotenv failed to initialize: {0}")]
    DotenvInitError(dotenv::Error),
    #[error("No Username provided or error decoding username: {0}")]
    NoUsername(dotenv::Error),
    #[error("No Password provided or error decoding password: {0}")]
    NoPassword(dotenv::Error),
    #[error("No Totp Secret provided or error decoding totp secret: {0}")]
    NoTotpSecret(dotenv::Error),
    #[error("We were unable to login with the provided credentials.")]
    UnableToLogin,
    #[error("No totp 2fa variant available.")]
    NoTotp2FAAvailable,
    #[error("Failed to get the current user: {0}")]
    GetCurrentUser(#[from] vrchatapi::apis::Error<vrchatapi::apis::authentication_api::GetCurrentUserError>),
    #[error("Failed to verify with 2fa: {0}")]
    Verify2FA(#[from] vrchatapi::apis::Error<vrchatapi::apis::authentication_api::Verify2FaError>),
    #[error("Failed to logout: {0}")]
    Logout(#[from] vrchatapi::apis::Error<vrchatapi::apis::authentication_api::LogoutError>),
    #[error("Failed to decode Totp from base32")]
    TOTPBase32,
}

pub type Result<T> = std::result::Result<T, anyhow::Error>;

const VERSION: &str = env!("CARGO_PKG_VERSION");
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().map_err(|e|Error::DotenvInitError(e))?;
    let username = dotenv::var("USERNAME").map_err(|e|Error::NoUsername(e))?;

    let client = {
        let password = dotenv::var("PASSWORD").map_err(|e|Error::NoPassword(e))?;

        let mut client = vrchatapi::apis::configuration::Configuration::default();
        client.user_agent = Some(format!("vrchatapi-rust@{VERSION} https://github.com/vrchatapi/vrchatapi-rust/issues/new"));
        client.basic_auth = Some((username.clone(), Some(password)));

        client
    };

    let u = login(&client, &username).await?;

    logout(&client).await?;
    Ok(())
}

async fn login(client: &vrchatapi::apis::configuration::Configuration, username: &String) -> Result<vrchatapi::models::CurrentUser> {
    let totp_secret = dotenv::var("TOTP_SECRET").map_err(|e|Error::NoTotpSecret(e))?;
    let totp_secret = base32::decode(Alphabet::Rfc4648Lower {padding: false}, totp_secret.as_str()).ok_or(Error::TOTPBase32)?;
    let generator = totp_rfc6238::TotpGenerator::new()
        .build();

    let u = match vrchatapi::apis::authentication_api::get_current_user(&client).await.map_err(|e|Error::GetCurrentUser(e))? {
        EitherUserOrTwoFactor::CurrentUser(u) => u,
        EitherUserOrTwoFactor::RequiresTwoFactorAuth(r2fa) => {
            if !r2fa.requires_two_factor_auth.contains(&"totp".to_string()) {
                let _ = logout(&client).await; //Ignore logout error. This is just here, to hopefully avoid session spam.
                return Err(Error::NoTotp2FAAvailable)?;
            }

            let code = generator.get_code(totp_secret.as_slice());
            println!("Generated code: {code}");
            if !vrchatapi::apis::authentication_api::verify2_fa(&client, TwoFactorAuthCode::new(code)).await.map_err(|e|Error::Verify2FA(e))?.verified {
                let _ = logout(&client).await; //Ignore logout error. This is just here, to hopefully avoid session spam.
                return Err(Error::UnableToLogin)?;
            }

            match vrchatapi::apis::authentication_api::get_current_user(&client).await.map_err(|e|Error::GetCurrentUser(e))? {
                EitherUserOrTwoFactor::CurrentUser(u) => u,
                EitherUserOrTwoFactor::RequiresTwoFactorAuth(_) => return Err(Error::UnableToLogin)?,
            }
        }
    };
    println!("Logged in as: {} (Login Name was {username}", u.username.as_ref().map(String::as_str).unwrap_or("Unknown User"));
    Ok(u)
}
async fn logout(client: &vrchatapi::apis::configuration::Configuration) -> Result<()> {
    vrchatapi::apis::authentication_api::logout(&client).await.map_err(|e|Error::Logout(e))?;
    Ok(())
}