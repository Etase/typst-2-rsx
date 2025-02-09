use std::io;

/// Custom error type `Error` representing possible errors during I/O operations and type conversions.
///
/// This enum includes the following variants:
///
/// - `Io`: Encapsulates an [`io::Error`], indicating an I/O operation error.
/// - `Convert`: Encapsulates a [`ConvertError`], indicating a type conversion error.
///
/// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
/// [`ConvertError`]: crate::ConvertError
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// SVG parsing error.
    #[error("SVG parsing error: {0}")]
    SvgParseError(#[from] serde_xml_rs::Error),

    /// Typst compilation error.
    #[error("Typst compile error: {0}")]
    TypstCompileError(#[from] io::Error),
}
