//! ## Validation rules

pub mod alphanumeric;
pub mod ascii;
pub mod byte_length;
pub mod contains;
#[cfg(feature = "credit-card")]
pub mod credit_card;
#[cfg(feature = "email")]
pub mod email;
pub mod inner;
pub mod ip;
pub mod length;
#[cfg(feature = "pattern")]
pub mod pattern;
#[cfg(feature = "phone-number")]
pub mod phone_number;
pub mod prefix;
pub mod range;
pub mod suffix;
#[cfg(feature = "url")]
pub mod url;
