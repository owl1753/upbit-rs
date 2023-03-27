pub mod api;
pub mod api_quotation;
pub mod constant;
pub mod response;
mod response_source;

pub use api::*;
pub use api_quotation::*;
pub use constant::*;
pub use response::*;

pub fn set_secret_key(secret_key: &str) {
    envmnt::set("SECRET_KEY", secret_key);
}

pub fn set_access_key(access_key: &str) {
    envmnt::set("ACCESS_KEY", access_key);
}