use super::request::Resp;
use super::types::{
    //{{{
    Payer,
    PaymentIntent,
    PaymentMethod,
    RedirectUrls,
    RequestNewPayment,
    Transaction,
    TransactionAmount,
    //}}}
};
use std::collections::HashMap;
// fn new{{{
pub fn new(
    bearer: &str,
    return_url: &str,
    cancel_url: &str,
    method: PaymentMethod,
    intent: PaymentIntent,
    transactions: Vec<Transaction>,
) -> Result<super::request::ResponseData, String>
{
    let ep = super::_make_endpoint("/payments/payment");
    let mut headers = HashMap::new();
    headers.insert("Authorization".into(), format!("Bearer {}", bearer).into());
    let body = RequestNewPayment {
        redirect_urls: RedirectUrls {
            return_url: return_url.into(),
            cancel_url: cancel_url.into(),
        },
        payer: Payer {
            payment_method: method,
        },
        intent: intent,
        transactions: transactions,
    };
    super::request::post_json(ep.as_str(), &mut headers, &body)
} //}}}
  //{{{ fn list
pub fn list(bearer: &str) -> Resp
{
    let mut headers = HashMap::new();
    headers.insert("Authorization".into(), format!("Bearer {}", bearer).into());
    super::request::get(
        super::_make_endpoint("/payments/payment").as_str(),
        &headers,
    )
} //}}}
  //{{{ fn execute
pub fn execute(bearer: &str, payment_id: &str, payer_id: &str) -> Resp
{
    let mut headers = HashMap::new();
    let mut body = headers.clone();
    headers.insert("Authorization".into(), format!("Bearer {}", bearer).into());
    body.insert("payer_id".into(), payer_id.into());
    super::request::post_json(
        super::_make_endpoint(format!("/payments/payment/{}/execute", payment_id).as_str())
            .as_str(),
        &mut headers,
        &body,
    )
} //}}}
