use chrono as chr;
use serde::{Deserialize, Serialize};

// paypals reply when requesting a new jwt
#[derive(Deserialize, Debug)]
pub struct ResponseAccessToken {
    scope: String,
    pub nonce: String,
    pub access_token: String,
    pub token_type: String,
    pub app_id: String,
    pub expires_in: i64,
}

#[derive(Serialize, Debug)]
pub struct RequestNewPayment {
    pub intent: PaymentIntent,
    pub payer: Payer,
    pub transactions: Vec<Transaction>,
    pub redirect_urls: RedirectUrls,
}

/// A single payment in PayPal's system, either completed or not.
#[derive(Deserialize, Debug)]
pub struct Payment {
    /// The ID of the payment.
    pub id: String,
    /// The payment intent.
    pub intent: PaymentIntent,
    /// The source of the funds for this payment. Payment method is PayPal Wallet payment or bank
    /// direct debit.
    pub payer: Payer,
    /// Use the application context resource to customize payment flow experience for your buyers.
    pub application_context: Option<ApplicationContext>,
    /// An array of payment-related transactions. A transaction defines what the payment is for and
    /// who fulfills the payment. For update and execute payment calls, the transactions object
    /// accepts the amount object only.
    pub transactions: Vec<Transaction>,
    /// The state of the payment.
    pub state: Option<State>,
    /// The PayPal-generated ID for the merchant's payment experience profile. For information, see
    /// [create web experience profile](https://developer.paypal.com/docs/api/payment-experience/#web-profiles_create).
    pub experience_profile_id: Option<String>,
    /// A free-form field that clients can use to send a note to the payer.
    pub note_to_payer: Option<String>,
    /// A set of redirect URLs that you provide for PayPal-based payments.
    pub redirect_urls: Option<RedirectUrls>,
    /// The reason code for a payment failure.
    pub failure_reason: Option<FailureReason>,
    /// The date and time when the payment was created.
    pub create_time: Option<chr::DateTime<chr::Utc>>,
    /// The date and time when the payment was updated.
    pub update_time: Option<chr::DateTime<chr::Utc>>,
    /// An array of request-related
    /// [HATEOAS links](https://developer.paypal.com/docs/api/reference/api-responses/#hateoas-links).
    pub links: Vec<LinkDescription>,
}

/// Used to customize the payment flow page.
#[derive(Deserialize, Debug)]
pub struct ApplicationContext {
    /// A label that overrides the business name in the merchant's PayPal account on the PayPal
    /// checkout pages.
    pub brand_name: Option<String>,
    /// The locale of pages that the PayPal payment experience displays. A valid value is AU, AT,
    /// BE, BR, CA, CH, CN, DE, ES, GB, FR, IT, NL, PL, PT, RU, or US. A five-character code is
    /// also valid for languages in these countries: da_DK, he_IL, id_ID, ja_JP, no_NO, pt_BR,
    /// ru_RU, sv_SE, th_TH, zh_CN, zh_HK, and zh_TW.
    pub locale: String,
    /// The type of landing page to show on the PayPal site for customer checkout. To use the
    /// non-PayPal account landing page, set to Billing. To use the PayPal account log in landing
    /// page, set to Login.
    pub landing_page: String,
    /// The shipping preference.
    pub shipping_preference: ShippingPreference,
    /// The user action.
    pub user_action: String,
}

/// The shipping preference.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShippingPreference {
    /// Redacts the shipping address from the PayPal pages. Recommended for digital goods.
    NoShipping,
    /// Uses the customer-selected shipping address on PayPal pages.
    GetFromFile,
    /// If available, uses the merchant-provided shipping address, which the customer cannot change
    /// on the PayPal pages. If the merchant does not provide an address, the customer can enter
    /// the address on PayPal pages.
    SetProvidedAddress,
}

/// A struct containing a url and some metadata.
#[derive(Deserialize, Debug)]
pub struct LinkDescription {
    /// The complete target URL. To make the related call, combine the method with this URI
    /// Template-formatted link. For pre-processing, include the `$`, `(`, and `)` characters. The
    /// href is the key HATEOAS component that links a completed call with a subsequent call.
    pub href: String,
    /// The link relation type, which serves as an ID for a link that unambiguously describes the
    /// semantics of the link.
    pub rel: String,
    /// The HTTP method required to make the related call.
    /// Possible values: `GET`, `POST`, `PUT`, `DELETE`, `HEAD`, `CONNECT`, `OPTIONS`, `PATCH`.
    pub method: Option<String>,
}

/// Returned when listing all payments in the system.
#[derive(Deserialize)]
pub struct ListPaymentResponse {
    /// A vector of the payments
    pub payments: Vec<Payment>,
    /// The number of payments, should be equal to `response.payments.len()`.
    pub count: usize,
    /// The ID of the element to use to get the next range of results.
    pub next_id: Option<String>,
}

/// Represents the state of a payment.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum State {
    /// The transaction was successfully created.
    Created,
    /// The customer approved the transaction. The state changes from created to approved on
    /// generation of the sale_id for sale transactions, authorization_id for authorization
    /// transactions, or order_id for order transactions.
    Approved,
    /// The transaction request failed.
    Failed,
}

/// The reason code for a payment failure.
#[allow(missing_docs)] // not documented by PayPal, but seems trivial
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FailureReason {
    UnableToCompleteTransaction,
    InvalidPaymentMethod,
    PayerCannotPay,
    CannotPayThisPayee,
    RedirectRequired,
    PayeeFilterRestrictions,
}

/// Struct containing urls where the users is redirected after visiting the paypal site.
#[derive(Serialize, Deserialize, Debug)]
pub struct RedirectUrls {
    /// The URL where the payer is redirected after he or she approves the payment.
    /// *Required for PayPal account payments.*
    pub return_url: String,
    /// The URL where the payer is redirected after he or she cancels the payment.
    /// *Required for PayPal account payments.*
    pub cancel_url: String,
}

/// The type of payment that is created
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntent {
    /// Makes an immediate payment.
    Sale,
    /// Authorizes a payment for capture later.
    Authorize,
    /// Creates an order.
    Order,
}

/// A paypal account that can be charged.
#[derive(Serialize, Deserialize, Debug)]
pub struct Payer {
    /// The payment method.
    pub payment_method: PaymentMethod,
    /// The status of payer's PayPal account
    pub status: Option<PayerStatus>,
    /// An array of a single funding instrument for the current payment. Valid only and required
    /// for the credit card payment method. The array must include either a credit_card or
    /// credit_card_token object. If the array contains more than one instrument, the payment is
    /// declined.
    pub funding_instruments: Option<Vec<FundingInstrument>>,
}

/// Represents one of the ways paypal is able to process payments.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    /// Credit card.
    CreditCard,
    /// A PayPal Wallet payment.
    Paypal,
    /// Pay upon invoice.
    PayUponInvoice,
    /// Carrier.
    Carrier,
    /// Alternate payment.
    AlternatePayment,
    /// Bank.
    Bank,
}

/// The status of a Payer
#[allow(missing_docs)] // undocumented by PayPal
#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayerStatus {
    Verified,
    Unverified,
}

/// A single transaction in paypals system. A payment consists of zero or more transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    /// The amount that is charged when this transaction is completed
    pub amount: TransactionAmount,
}

/// A currency-aware representation of an amount of money
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionAmount {
    /// The ISO 4217 currency code, for example "USD" or "EUR"
    pub currency: String,
    /// The amount of currency that is to be charge, for example "10" or "12.34"
    pub total: String,
}

/// Data about a credit card.
#[derive(Deserialize)]
pub struct FundingInstrument {
    /// Full representation of a credit card
    pub credit_card: Option<CreditCard>,
    /// Token representation of a credit card
    pub credit_card_token: Option<CreditCardToken>,
}

/// A credit card
#[derive(Deserialize)]
pub struct CreditCard {
    /// The credit card number. Value is numeric characters only with no spaces or punctuation.
    /// Must conform to the modulo and length required by each credit card type. Redacted in
    /// responses.
    pub number: String,
    /// The credit card type. Value is visa, mastercard, discover, or amex. Do not use these
    /// lowercase values for display.
    #[serde(rename = "type")]
    pub _type: String,
    /// The expiration month with no leading zero. Value is from 1 to 12.
    pub expire_month: i32,
    /// The four-digit expiration year.
    pub expire_year: i32,
    /// The three- to four-digit card validation code.
    pub cvv2: Option<String>,
    /// The card holder's first name.
    pub first_name: Option<String>,
    /// The card holder's last name.
    pub last_name: Option<String>,
    /// The billing address for this card.
    pub billing_address: Option<Address>,
    /// An array of request-related HATEOAS links.
    pub links: Vec<LinkDescription>,
}

/// Represents an address.
#[derive(Deserialize)]
pub struct Address {
    /// The first line of the address. For example, number, street, and so on.
    pub line1: String,
    /// The second line of the address. For example, suite or apartment number.
    pub line2: Option<String>,
    /// The city name.
    pub city: Option<String>,
    /// The two-character ISO 3166-1 code that identifies the country or region.
    pub country_code: String,
    /// The postal code, which is the zip code or equivalent. Typically required for countries with
    /// a postal code or an equivalent.
    pub postal_code: Option<String>,
    /// The code for a US state or the equivalent for other countries. Required for transactions if
    /// the address is in one of these countries: Argentina, Brazil, Canada, China, India, Italy,
    /// Japan, Mexico, Thailand, or United States. Maximum length is 40 single-byte characters.
    pub state: Option<String>,
    /// The phone number, in E.123 format. Maximum length is 50 characters.
    pub phone: Option<String>,
    /// The address normalization status.
    pub normalization_status: Option<NormalizationStatus>,
    /// The type of address. For example, HOME_OR_WORK, GIFT, and so on.
    #[serde(rename = "type")]
    pub _type: Option<String>,
}

/// The address normalization status. Returned only for payers from Brazil.
#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NormalizationStatus {
    /// Unknown
    Unknown,
    /// Unnormalized user preferred.
    UnnormalizedUserPreferred,
    /// Normalized.
    Normalized,
    /// Unnormalized.
    Unnormalized,
}

/// A credit card in token representation.
#[derive(Deserialize)]
pub struct CreditCardToken {
    /// The ID of credit card that is stored in the PayPal vault.
    pub credit_card_id: String,
    /// A unique ID that you can assign and track when you store a credit card in the vault or use
    /// a vaulted credit card. This ID can help to avoid unintentional use or misuse of credit
    /// cards and can be any value, such as a UUID, user name, or email address. *Required* when
    /// you use a vaulted credit card and if a payer_id was originally provided when you vaulted
    /// the credit card.
    pub payer_id: Option<String>,
    /// The last four digits of the stored credit card number.
    pub last4: Option<String>,
    /// The credit card type. Value is visa, mastercard, discover, or amex. Do not use these
    /// lowercase values for display.
    #[serde(rename = "type")]
    pub _type: Option<String>,
    /// The expiration month with no leading zero. Value is from 1 to 12.
    pub expire_month: Option<i32>,
    /// The four-digit expiration year.
    pub expire_year: Option<i32>,
}
