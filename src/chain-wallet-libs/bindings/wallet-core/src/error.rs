use std::{
    error,
    fmt::{self, Display},
};

/// result returned by a call, this allows to check if an error
/// occurred while executing the function.
///
/// if an error occurred it is then possible to collect more information
/// about the kind of error as well as details from the underlying libraries
/// that may be useful in case of bug reports or to figure out why the inputs
/// were not valid and resulted in the function to return with error
pub struct Result(std::result::Result<(), Error>);

/// The error structure, contains details of what may have gone wrong
///
/// See the error's kind for the high level information of what went wrong,
/// it is also possible to then extract details (if any) of the error.
///
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    details: Option<Box<dyn error::Error + 'static>>,
}

/// a code representing the kind of error that occurred
#[repr(u32)]
pub enum ErrorCode {
    /// the input was not valid, it may be because it was a null pointer
    /// where it was expected to be already allocated. See the details
    /// for more information.
    ///
    /// When this kind of error occurs it is likely a developer flow issue
    /// rather than a user input issue. See the error details for more
    /// details.
    InvalidInput = 1,

    /// the operation on the wallet conversion object fail, it may be
    /// an out of bound operation when attempting to access the `nth`
    /// transaction of the conversion.
    WalletConversion = 2,
}

#[derive(Debug)]
pub enum ErrorKind {
    /// kind of error where the input (named `argument_name`) is of
    /// invalid format or of unexpected value (null pointer).
    ///
    /// The `details` should provide more info on what caused the error.
    InvalidInput { argument_name: &'static str },

    /// This is the kind of error that may happen when operating
    /// the transactions of the wallet conversion. For example,
    /// there may be an out of bound error
    WalletConversion,
}

impl ErrorKind {
    /// retrieve the error code associated to the error kind
    ///
    /// useful to extract the kind of error that occurred in a portable
    /// way, without to use string encoded version of the `ErrorKind`.
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::InvalidInput { .. } => ErrorCode::InvalidInput,
            Self::WalletConversion => ErrorCode::WalletConversion,
        }
    }
}

impl Error {
    /// access the error kind
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// if there are details return the pointer to the error type that triggered
    /// the error.
    ///
    /// this is useful to display more details as to why an error occurred.
    pub fn details(&self) -> Option<&(dyn error::Error + 'static)> {
        self.details.as_ref().map(|boxed| boxed.as_ref())
    }
}

impl Result {
    /// returns `true` if the `Result` means success
    pub fn is_ok(&self) -> bool {
        self.0.is_ok()
    }

    /// returns `true` if the `Result` means error
    pub fn is_err(&self) -> bool {
        self.0.is_err()
    }

    /// if it is an error, this function will returns the the error object,
    /// otherwise it will return `None`
    pub fn error(&self) -> Option<&Error> {
        self.0.as_ref().err()
    }

    /// constructor to build a `Result` that means success
    ///
    /// # example
    ///
    /// ```
    /// # use wallet_core::Result;
    ///
    /// let result = Result::success();
    ///
    /// assert!(result.is_ok());
    /// # assert!(!result.is_err());
    /// ```
    pub fn success() -> Self {
        Self(Ok(()))
    }

    /// set some details to the `Result` object if the `Result` is of
    /// error kind
    ///
    /// If the `Result` means success, then nothing is returned.
    ///
    /// # Example
    ///
    /// ```
    /// # use wallet_core::Result;
    /// # use thiserror::Error;
    /// # #[derive(Error, Debug)]
    /// # #[error("Unexpected null pointer")]
    /// # struct NulPointer;
    /// fn example(pointer: *mut u8) -> Result {
    ///   if pointer.is_null() {
    ///     Result::invalid_input("pointer")
    ///   } else {
    ///     Result::success()
    ///   }
    /// }
    ///
    /// let mut input = 2;
    /// let input: *mut u8 = &mut 2;
    /// let result = example(input).details(NulPointer);
    ///
    /// # assert!(!result.is_err());
    /// assert!(result.is_ok());
    /// ```
    ///
    pub fn details<E>(self, details: E) -> Self
    where
        E: error::Error + 'static,
    {
        match self.0 {
            Ok(()) => Self::success(),
            Err(mut err) => {
                err.details = Some(Box::new(details));
                Self(Err(err))
            }
        }
    }

    /// construct a Result which is an error with invalid inputs
    ///
    /// `argument_name` is expected to be a pointer to the parameter name.
    ///
    /// # example
    ///
    /// ```
    /// # use wallet_core::Result;
    /// # use thiserror::Error;
    /// # #[derive(Error, Debug)]
    /// # #[error("Unexpected null pointer")]
    /// # struct NulPointer;
    /// fn example(pointer: *mut u8) -> Result {
    ///   if pointer.is_null() {
    ///     Result::invalid_input("pointer")
    ///         .details(NulPointer)
    ///   } else {
    ///     Result::success()
    ///   }
    /// }
    ///
    /// let result = example(std::ptr::null_mut());
    ///
    /// assert!(result.is_err());
    /// # assert!(!result.is_ok());
    /// ```
    pub fn invalid_input(argument_name: &'static str) -> Self {
        Self(Err(Error {
            kind: ErrorKind::InvalidInput { argument_name },
            details: None,
        }))
    }

    pub fn wallet_conversion() -> Self {
        Self(Err(Error {
            kind: ErrorKind::WalletConversion,
            details: None,
        }))
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput { argument_name } => {
                write!(f, "The argument '{}' is invalid.", argument_name)
            }
            Self::WalletConversion => {
                f.write_str("Error while performing operation on the wallet conversion object")
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.details()
    }
}
