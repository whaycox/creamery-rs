mod request;
mod method;
mod version;
mod headers;
mod response;
mod status;
mod parsing;
mod error;

pub use request::*;
pub use method::*;
pub use version::*;
pub use headers::*;
pub use response::*;
pub use status::*;
pub use parsing::*;
pub use error::*;

pub type CurdsWebResult<T> = Result<T, CurdsWebError>;