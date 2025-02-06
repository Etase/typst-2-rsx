use std::fmt;

/// `ConvertError` is a custom error type used to represent an error that occurs during conversion.
/// This error contains a `details` field that stores the detailed error information.
#[derive(Debug)]
pub struct ConvertError {
    pub details: String,
}

impl ConvertError {
    /// Creates a new instance of `ConvertError`
    ///
    /// # parameter
    ///
    /// * `msg` - Detailed description of the error.
    ///
    /// # Return value
    ///
    /// Returns a new instance of `ConvertError`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use typst_2_rsx::error::ConvertError;
    ///
    /// let error = ConvertError::new("An error occurred");
    /// assert_eq!(error.details, "An error occurred");
    /// ```
    pub fn new(msg: &str) -> ConvertError {
        ConvertError {
            details: msg.to_string(),
        }
    }
}

/// implements the `fmt::Display` trait so that `ConvertError` can be printed in a friendly format.
///
/// # Example
///
/// ```rust
/// use typst_2_rsx::error::ConvertError;
///
/// let error = ConvertError::new("Some conversion error");
/// println!("{}", error);   // Output: MyError: Some conversion error
/// ```
impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyError: {}", self.details)
    }
}
