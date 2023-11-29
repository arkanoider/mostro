use crate::cli::settings::MostroSettingsError;
use crate::db::MostroDatabaseError;
use crate::util::MostroAppError;
use std::fmt;

// #[derive(Debug, PartialEq, Eq)]
// pub struct MostroError{
//     pub kind: MostroGlobalError,
// }

// #[derive(Debug, PartialEq, Eq)]
// pub enum MostroAppError {
//     AddInvoice,
//     AdminCancel,
//     AdminSettle,
//     Cancel,
//     Dispute,
//     FiatSent,
//     Order,
//     RateUser,
//     Release,
//     TakeBuy,
//     TakeSell,
// }

#[derive(Debug, PartialEq, Eq)]
pub enum MostroError {
    Database(MostroDatabaseError),
    MostroAppError(MostroAppError),
    MostroInitError(MostroSettingsError),
    // MinExpirationTimeError,
    // MinAmountError,
    // WrongAmountError,
    // NoAPIResponse,
}

// impl std::error::Error for MostroError {}

impl fmt::Display for MostroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MostroError::Database(_) => write!(f, "Database error"),
            MostroError::MostroAppError(_) => write!(f, "Application error"),
            MostroError::MostroInitError(_) => write!(f, "Init error"),
            // MostroError::InvoiceExpiredError => write!(f, "Invoice has expired"),
            // MostroError::MinExpirationTimeError => write!(f, "Minimal expiration time on invoice"),
            // MostroError::MinAmountError => write!(f, "Minimal payment amount"),
            // MostroError::WrongAmountError => write!(f, "The amount on this invoice is wrong"),
            // MostroError::NoAPIResponse => write!(f, "Price API not answered - retry"),
        }
    }
}

// impl From<lightning_invoice::Bolt11ParseError> for MostroError {
//     fn from(_: lightning_invoice::Bolt11ParseError) -> Self {
//         MostroError::ParsingInvoiceError
//     }
// }

// impl From<lightning_invoice::ParseOrSemanticError> for MostroError {
//     fn from(_: lightning_invoice::ParseOrSemanticError) -> Self {
//         MostroError::ParsingInvoiceError
//     }
// }

// impl From<std::num::ParseIntError> for MostroError {
//     fn from(_: std::num::ParseIntError) -> Self {
//         MostroError::ParsingNumberError
//     }
// }

// impl From<reqwest::Error> for MostroError {
//     fn from(_: reqwest::Error) -> Self {
//         MostroError::NoAPIResponse
//     }
// }
