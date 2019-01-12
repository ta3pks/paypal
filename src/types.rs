#![allow(bad_style)]
//
// ResponseAccessToken {{{
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseAccessToken
{
    scope: String,
    pub nonce: String,
    pub access_token: String,
    pub token_type: String,
    pub app_id: String,
    pub expires_in: i64,
} //}}}
  //RequestNewPayment {{{
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestNewPayment
{
    pub intent: PaymentIntent,
    pub payer: Payer,
    pub transactions: Vec<Transaction>,
    pub redirect_urls: RedirectUrls,
} //}}}
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseNewPayment {
    pub id:String,
    pub intent:PaymentIntent,
    pub payer:Payer,
    pub state:State,
    pub failure_reason:Failure_reason,
    pub create_time:String,
    pub update_time:String,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum State {
    created,
    approved,
    failed
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Failure_reason {
    UNABLE_TO_COMPLETE_TRANSACTION,
    INVALID_PAYMENT_METHOD,
    PAYER_CANNOT_PAY,
    CANNOT_PAY_THIS_PAYEE,
    REDIRECT_REQUIRED,
    PAYEE_FILTER_RESTRICTIONS
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RedirectUrls
{
    pub return_url: String,
    pub cancel_url: String,
}
// intent{{{
#[derive(Serialize, Deserialize, Debug)]
pub enum PaymentIntent
{
    sale,
    authorize,
    order,
} //}}}
  // payer struct {{{
#[derive(Serialize, Deserialize, Debug)]
pub struct Payer
{
    pub payment_method: PaymentMethod,
} //}}}
  // PaymentMethod struct {{{
#[derive(Serialize, Deserialize, Debug)]
pub enum PaymentMethod
{
    credit_card,
    paypal,
    pay_upon_invoice,
    carrier,
    alternate_payment,
    bank,
} //}}}
  // Transaction struct {{{
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction
{
    pub amount: TransactionAmount,
} //}}}
  // TransactionAmount struct{{{
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionAmount
{
    pub currency: String,
    pub total: String,
} //}}}
