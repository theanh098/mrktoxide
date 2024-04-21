mod cosmos;
mod http;

static PALLET_API_URL: &'static str = "https://api.prod.pallet.exchange/api";

pub type ServiceError = reqwest::Error;
pub use cosmos::*;
pub use http::*;
