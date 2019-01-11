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
