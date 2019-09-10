use crate::request::Resp;
use crate::types::Payment;
use crate::types::{
    ListPaymentResponse, Payer, PaymentIntent, PaymentMethod, RedirectUrls, RequestNewPayment,
    Transaction,
};
use std::collections::HashMap;

/// Use this endpoint to create a new payment.
/// You can obtain a new bearer token using the `get_token` function provided.
/// ```rust,no_run
/// use paypal::{get_token, payment};
/// use paypal::{PaymentMethod, PaymentIntent, Transaction, TransactionAmount};
///
/// let token = get_token("my_id", "my_secret").unwrap();
/// let amount = TransactionAmount {
///     currency: "USD".to_string(),
///     total: "100.00".to_string()
/// };
/// let new_payment = payment::new(
///     &token.access_token,
///     "mysite.com/whooyoupaid",
///     "mysite.com/nooyoufailed",
///     PaymentMethod::Paypal,
///     PaymentIntent::Sale,
///     vec![Transaction { amount }],
/// ).unwrap();
/// ```
pub fn new(
    bearer: &str,
    return_url: &str,
    cancel_url: &str,
    method: PaymentMethod,
    intent: PaymentIntent,
    transactions: Vec<Transaction>,
) -> Resp<Payment> {
    let ep = crate::_make_endpoint("/payments/payment");
    let mut headers = HashMap::new();
    headers.insert("Authorization".into(), format!("Bearer {}", bearer));
    let body = RequestNewPayment {
        redirect_urls: RedirectUrls {
            return_url: return_url.into(),
            cancel_url: cancel_url.into(),
        },
        payer: Payer {
            payment_method: method,
            funding_instruments: None,
            status: None,
        },
        intent,
        transactions,
    };
    crate::request::post_json(ep.as_str(), &mut headers, &body)
}

/// Returns a list of all transactions made using the account that corresponds to the
/// provided bearer token.
///
/// ```rust,no_run
/// let token = get_token("my_id", "my_secret").unwrap();
/// let list = payment::list(&token).unwrap();
/// ```
pub fn list(bearer: &str) -> Resp<ListPaymentResponse> {
    let mut headers = HashMap::new();
    headers.insert("Authorization".into(), format!("Bearer {}", bearer));
    crate::request::get(
        crate::_make_endpoint("/payments/payment").as_str(),
        &headers,
    )
}

/// Finalizes charging of a previously constructed payment. This usually comes after the payment
/// has been created and _approved_ by the customer.
///
/// ```rust,no_run
/// use paypal::{get_token, payment};
/// use paypal::{PaymentMethod, PaymentIntent, Transaction, TransactionAmount};
///
/// let token = get_token("my_id", "my_secret").unwrap();
/// let amount = TransactionAmount {
///     currency: "USD".to_string(),
///     total: "100.00".to_string()
/// };
/// let new_payment = payment::new(
///     &token.access_token,
///     "mysite.com/whooyoupaid",
///     "mysite.com/nooyoufailed",
///     PaymentMethod::Paypal,
///     PaymentIntent::Sale,
///     vec![Transaction { amount }],
/// ).unwrap();
/// // Have the user approve the payment here, using the webpage in payment.links, for example:
/// let payer_id = function_that_sends_user_to_webpage(&new_payment);
/// let finalized_payment = payment::execute(&token, &new_payment.id, &payer_id).unwrap();
/// ```
pub fn execute(bearer: &str, payment_id: &str, payer_id: &str) -> Resp<Payment> {
    let mut headers = HashMap::new();
    let mut body = headers.clone();
    headers.insert("Authorization".into(), format!("Bearer {}", bearer));
    body.insert("payer_id".into(), payer_id.into());
    crate::request::post_json(
        &crate::_make_endpoint(&format!("/payments/payment/{}/execute", payment_id)).as_str(),
        &mut headers,
        &body,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::io::{self, BufRead};

    lazy_static! {
        static ref CLIENT_ID: String = {
            dotenv::dotenv().ok();
            std::env::var("CLIENT_ID").expect("`CLIENT_ID` env parameter to be set")
        };
        static ref SECRET: String = {
            dotenv::dotenv().ok();
            std::env::var("SECRET").expect("`SECRET` env parameter to be set")
        };
    }

    #[test]
    fn test_new() {
        let token = crate::get_token(&CLIENT_ID, &SECRET).unwrap(); // alos checked by other test
        let amount = crate::types::TransactionAmount {
            currency: "USD".to_string(),
            total: "100.00".to_string(),
        };
        new(
            &token.access_token,
            "mysite.com/whooyoupaid",
            "mysite.com/nooyoufailed",
            PaymentMethod::Paypal,
            PaymentIntent::Sale,
            vec![Transaction { amount }],
        )
        .unwrap();
    }

    #[test]
    fn test_list() {
        let token = crate::get_token(&CLIENT_ID, &SECRET).unwrap();
        let _list = list(&token.access_token).unwrap();
    }
}
