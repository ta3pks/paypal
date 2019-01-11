mod token;
mod types;
mod request;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
pub use self::token::get_token;
const _ADDR: &str = "https://api.sandbox.paypal.com/v1";
fn _make_endpoint(ep: &str) -> String //{{{
{
    let mut _ep = String::from(_ADDR);
    _ep.push_str(ep);
    _ep
} //}}}
