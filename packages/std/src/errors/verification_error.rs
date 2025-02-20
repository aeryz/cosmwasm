#[cfg(all(feature = "backtraces", feature = "std"))]
use std::backtrace::Backtrace;

#[cfg(all(not(target_arch = "wasm32"), feature = "std"))]
use cosmwasm_crypto::CryptoError;

#[cfg_attr(feature = "std", derive(thiserror::Error))]
#[derive(Debug)]
pub enum VerificationError {
    #[cfg_attr(feature = "std", error("Batch error"))]
    BatchErr,
    #[cfg_attr(feature = "std", error("Generic error"))]
    GenericErr,
    #[cfg_attr(feature = "std", error("Invalid hash format"))]
    InvalidHashFormat,
    #[cfg_attr(feature = "std", error("Invalid signature format"))]
    InvalidSignatureFormat,
    #[cfg_attr(feature = "std", error("Invalid public key format"))]
    InvalidPubkeyFormat,
    #[cfg_attr(
        feature = "std",
        error("Invalid recovery parameter. Supported values: 0 and 1.")
    )]
    InvalidRecoveryParam,
    #[cfg_attr(feature = "std", error("Unknown error: {error_code}"))]
    UnknownErr {
        error_code: u32,
        #[cfg(all(feature = "backtraces", feature = "std"))]
        backtrace: Backtrace,
    },
}

impl VerificationError {
    pub fn unknown_err(error_code: u32) -> Self {
        VerificationError::UnknownErr {
            error_code,
            #[cfg(all(feature = "backtraces", feature = "std"))]
            backtrace: Backtrace::capture(),
        }
    }
}

impl PartialEq<VerificationError> for VerificationError {
    fn eq(&self, rhs: &VerificationError) -> bool {
        match self {
            VerificationError::BatchErr => matches!(rhs, VerificationError::BatchErr),
            VerificationError::GenericErr => matches!(rhs, VerificationError::GenericErr),
            VerificationError::InvalidHashFormat => {
                matches!(rhs, VerificationError::InvalidHashFormat)
            }
            VerificationError::InvalidPubkeyFormat => {
                matches!(rhs, VerificationError::InvalidPubkeyFormat)
            }
            VerificationError::InvalidSignatureFormat => {
                matches!(rhs, VerificationError::InvalidSignatureFormat)
            }
            VerificationError::InvalidRecoveryParam => {
                matches!(rhs, VerificationError::InvalidRecoveryParam)
            }
            VerificationError::UnknownErr { error_code, .. } => {
                if let VerificationError::UnknownErr {
                    error_code: rhs_error_code,
                    ..
                } = rhs
                {
                    error_code == rhs_error_code
                } else {
                    false
                }
            }
        }
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "std"))]
impl From<CryptoError> for VerificationError {
    fn from(original: CryptoError) -> Self {
        match original {
            CryptoError::InvalidHashFormat { .. } => VerificationError::InvalidHashFormat,
            CryptoError::InvalidPubkeyFormat { .. } => VerificationError::InvalidPubkeyFormat,
            CryptoError::InvalidSignatureFormat { .. } => VerificationError::InvalidSignatureFormat,
            CryptoError::GenericErr { .. } => VerificationError::GenericErr,
            CryptoError::InvalidRecoveryParam { .. } => VerificationError::InvalidRecoveryParam,
            CryptoError::BatchErr { .. } => VerificationError::BatchErr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // constructors
    #[test]
    fn unknown_err_works() {
        let error = VerificationError::unknown_err(123);
        match error {
            VerificationError::UnknownErr { error_code, .. } => assert_eq!(error_code, 123),
            _ => panic!("wrong error type!"),
        }
    }
}
