use data_encoding::DecodeKind;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum OtpError {
    SecretEncoding(DecodeKind, usize), // Secret encoding error, of given kind at give position
    MissingPin,                        // Missing Pin for Yandex / MOTP Codes
    ShortSecret,                       // Short secret for Yandex codes
    MissingCounter,                    // Missing counter for HOTP codes
    InvalidOffset,                     // Invalid offset
    InvalidDigest,                     // Invalid digest
    InvalidDigits,                     // Invalid Digits value (too high or low)
}

impl Display for OtpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OtpError::SecretEncoding(decode_kind, position) => {
                f.write_str(format!("Decode error {decode_kind} at {position}").as_str())
            }
            OtpError::MissingPin => f.write_str("Missing pin value"),
            OtpError::MissingCounter => f.write_str("Missing counter value"),
            OtpError::InvalidDigest => f.write_str("Invalid digest"),
            OtpError::InvalidOffset => f.write_str("Invalid offset"),
            OtpError::ShortSecret => f.write_str("Secret length less than 16 bytes"),
            OtpError::InvalidDigits => f.write_str("Digits value too high or low"),
        }
    }
}

impl std::error::Error for OtpError {}
