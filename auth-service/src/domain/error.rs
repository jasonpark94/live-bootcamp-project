pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnauthorizedCredentials,
    MisinformedCredentials,
    UnexpectedError,
}