const _ADDR: &str = "https://api.sandbox.paypal.com/v1";
fn _make_endpoint(ep: &str) -> String //{{{
{
    let mut _ep = String::from(_ADDR);
    _ep.push_str(ep);
    _ep
} //}}}
