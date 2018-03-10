use std::{error, fmt};

#[derive(Debug, Clone)]
// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
pub struct PublicKeyParseError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the
// display style.
//
// Note that we don't store any extra info about the errors. This means we
// can't state which string failed to parse without modifying our types to
// carry that information.
impl fmt::Display for PublicKeyParseError {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "String wasn't public key") }
}

// This is important for other errors to wrap this one.
impl error::Error for PublicKeyParseError {
   fn description(&self) -> &str { "String wasn't public key" }

   fn cause(&self) -> Option<&error::Error> {
      // Generic error, underlying cause isn't tracked.
      None
   }
}
