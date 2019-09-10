#![deny(missing_docs)]

//! A crate that serves to abstract away the network aspect of interfacing with paypal.
//! Created by nikos
//!
//! Creating a payment is done using the `create_payment` function.
//! ```rust,no_run
//! let token = paypal::get_token("my_id", "my_secret")?;
//! let amount = TransactionAmount {
//!     currency: "USD".to_string(),
//!     total: "100.00".to_string()
//! };
//! let new_payment = payment::new(
//!     &token.access_token,
//!     "mysite.com/whooyoupaid",
//!     "mysite.com/nooyoufailed",
//!     PaymentMethod::Paypal,
//!     PaymentIntent::Sale,
//!     vec![Transaction { amount }],
//! )?;
//! ```

/// Possible ways that a paypal request can fail
pub mod error;
mod payment;
mod request;
mod token;

mod types;

pub use token::get_token;
pub use types::{
    Address, ApplicationContext, CreditCard, CreditCardToken, FailureReason, FundingInstrument,
    LinkDescription, ListPaymentResponse, NormalizationStatus, Payer, PayerStatus, Payment,
    PaymentIntent, PaymentMethod, RedirectUrls, State, Transaction, TransactionAmount,
};
pub use payment::*;

#[cfg(feature = "test-mode")]
const _ADDR: &str = "https://api.sandbox.paypal.com/v1";

#[cfg(not(feature = "test-mode"))]
const _ADDR: &str = "https://api.paypal.com/v1";

fn _make_endpoint(ep: &str) -> String {
    format!("{}{}", _ADDR, ep)
}
