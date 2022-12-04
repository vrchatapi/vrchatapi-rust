impl std::fmt::Display for InviteMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message => write!(f, "message"),
            Self::Response => write!(f, "response"),
            Self::Request => write!(f, "request"),
            Self::RequestResponse => write!(f, "requestResponse"),
        }
    }
}