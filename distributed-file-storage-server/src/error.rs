use super::{fmt, DieselError, MultipartError, RecvError, ResponseError, SendError, StdError};

/// Define a custom error type for handling different kinds of errors in the order book
#[derive(Debug)] // Enable debug formatting for the enum
pub enum CustomError {
    // Error caused by issues in I/O operations (e.g., reading/writing from/to stdin, files, etc.)
    IoError(StdError),

    // Error when parsing a value (e.g., price or volume) fails
    ParseError(MultipartError),

    // Error when a WebSocket connection fails or an issue occurs during communication
    ConnectionError(r2d2::Error),

    // Error when sending a message over the channel fails
    SendError(String),

    // Error when receiving a message over the channel fails
    RecvError(RecvError),

    DatabaseError(DieselError),

    NullError,
}

/// Implement the `Display` trait for the `CustomError` enum
/// This allows us to convert the errors into human-readable strings for easy debugging and logging
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Convert the `IoError` into a string with "I/O Error" prefix
            CustomError::IoError(e) => write!(f, "I/O Error: {}", e),

            // Generic parse error with a custom message
            CustomError::ParseError(e) => write!(f, "Parse error: {}", e),

            // Error when there's a WebSocket connection issue
            CustomError::ConnectionError(e) => write!(f, "Connection error: {}", e),

            // Error when a message can't be sent over a channel
            CustomError::SendError(e) => write!(f, "Send error: {}", e),

            CustomError::RecvError(e) => write!(f, "Receive error: {}", e),

            CustomError::DatabaseError(e) => write!(f, "Database error: {}", e),

            CustomError::NullError => write!(f, "Value is Null"),
        }
    }
}

/// Implement `From` trait for automatic conversion from `std::io::Error` to `CustomError::IoError`
/// This allows using the `?` operator in functions that return `Result<(), CustomError>`
impl From<StdError> for CustomError {
    fn from(error: StdError) -> Self {
        // Convert the `std::io::Error` into `CustomError::IoError`
        CustomError::IoError(error)
    }
}

impl From<CustomError> for StdError {
    fn from(error: CustomError) -> Self {
        StdError::new(std::io::ErrorKind::Other, error.to_string())
    }
}

impl From<r2d2::Error> for CustomError {
    fn from(error: r2d2::Error) -> Self {
        CustomError::ConnectionError(error)
    }
}

impl From<SendError> for CustomError {
    fn from(error: SendError) -> Self {
        CustomError::SendError(error.to_string())
    }
}

impl From<RecvError> for CustomError {
    fn from(error: RecvError) -> Self {
        CustomError::RecvError(error)
    }
}

impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> Self {
        CustomError::DatabaseError(error)
    }
}

impl From<MultipartError> for CustomError {
    fn from(error: MultipartError) -> Self {
        CustomError::ParseError(error)
    }
}

impl ResponseError for CustomError {}
