pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnauthorizedCredentials,
    MisinformedCredentials,
    Unexpected,
    MissingToken,
    InvalidToken,
}