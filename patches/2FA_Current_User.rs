#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EitherUserOrTwoFactor{
    CurrentUser(CurrentUser),
    RequiresTwoFactorAuth(RequiresTwoFactorAuth),
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RequiresTwoFactorAuth{
    #[serde(rename = "requiresTwoFactorAuth")]
    pub requires_two_factor_auth: Vec<String>
}