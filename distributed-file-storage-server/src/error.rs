use super::{fmt, DieselError, MultipartError, RecvError, ResponseError, SendError, StdError};

/// Define a custom error type to handle various error cases in the order book system
#[derive(Debug)] // Automatically derive the Debug trait for easier debugging
pub enum CustomError {
    // I/O error, such as reading or writing to a file or standard input/output
    IoError(StdError),

    // Error related to parsing multipart data (e.g., file uploads, form data)
    ParseError(MultipartError),

    // Error caused by failure in establishing or maintaining WebSocket connections
    ConnectionError(r2d2::Error),

    // Error when sending a message through a channel fails
    SendError(String),

    // Error when receiving a message from a channel fails
    RecvError(RecvError),

    // Error related to database operations (Diesel ORM specific errors)
    DatabaseError(DieselError),

    // Error when a null value is encountered where it shouldn't be
    NullError,
}

/// Implement the `Display` trait for the `CustomError` enum
/// This allows the errors to be displayed as human-readable messages, useful for debugging/logging
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // For I/O errors, display "I/O Error" followed by the underlying error message
            CustomError::IoError(e) => write!(f, "I/O Error: {}", e),

            // For parsing errors, display "Parse error" followed by the error details
            CustomError::ParseError(e) => write!(f, "Parse error: {}", e),

            // For WebSocket connection errors, display "Connection error"
            CustomError::ConnectionError(e) => write!(f, "Connection error: {}", e),

            // For channel send errors, display "Send error" with the associated error message
            CustomError::SendError(e) => write!(f, "Send error: {}", e),

            // For channel receive errors, display "Receive error" with the error details
            CustomError::RecvError(e) => write!(f, "Receive error: {}", e),

            // For database-related errors, display "Database error" with the underlying error message
            CustomError::DatabaseError(e) => write!(f, "Database error: {}", e),

            // For null value errors, simply display "Value is Null"
            CustomError::NullError => write!(f, "Value is Null"),
        }
    }
}

/// Implement the `From` trait to allow automatic conversion of `std::io::Error` to `CustomError::IoError`
/// This simplifies error handling with the `?` operator, which can convert one error type into another
impl From<StdError> for CustomError {
    fn from(error: StdError) -> Self {
        // Convert `std::io::Error` into our custom `IoError` variant
        CustomError::IoError(error)
    }
}

/// Implement the `From` trait to convert `CustomError` back into a `std::io::Error`
/// This allows integration with standard library functions that expect `std::io::Error`
impl From<CustomError> for StdError {
    fn from(error: CustomError) -> Self {
        // Convert `CustomError` to `std::io::Error`, setting the error kind to "Other"
        StdError::new(std::io::ErrorKind::Other, error.to_string())
    }
}

/// Implement the `From` trait to convert `r2d2::Error` (connection pooling error) into `CustomError::ConnectionError`
impl From<r2d2::Error> for CustomError {
    fn from(error: r2d2::Error) -> Self {
        // Convert the `r2d2::Error` to `CustomError::ConnectionError`
        CustomError::ConnectionError(error)
    }
}

/// Implement the `From` trait to convert `SendError` (channel send error) into `CustomError::SendError`
impl From<SendError> for CustomError {
    fn from(error: SendError) -> Self {
        // Convert `SendError` into a `CustomError::SendError` and store the error as a string
        CustomError::SendError(error.to_string())
    }
}

/// Implement the `From` trait to convert `RecvError` (channel receive error) into `CustomError::RecvError`
impl From<RecvError> for CustomError {
    fn from(error: RecvError) -> Self {
        // Convert `RecvError` into `CustomError::RecvError`
        CustomError::RecvError(error)
    }
}

/// Implement the `From` trait to convert `DieselError` (database-related error) into `CustomError::DatabaseError`
impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> Self {
        // Convert `DieselError` into `CustomError::DatabaseError`
        CustomError::DatabaseError(error)
    }
}

/// Implement the `From` trait to convert `MultipartError` (error in multipart form parsing) into `CustomError::ParseError`
impl From<MultipartError> for CustomError {
    fn from(error: MultipartError) -> Self {
        // Convert `MultipartError` into `CustomError::ParseError`
        CustomError::ParseError(error)
    }
}

/// Automatically implement `ResponseError` for `CustomError`
/// This enables using `CustomError` as a valid response error type in web applications
impl ResponseError for CustomError {}
