pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnauthorizedCredentials,
    IncorrectCredentials,
    MisinformedCredentials,
    Unexpected,
    MissingToken,
    InvalidToken,
}