use std::fmt::Display;

#[derive(Debug)]
#[repr(u16)]
pub enum HttpStatus {
    Continue = 100,
    SwitchingProtocols = 101,

    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,

    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    ContentTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    MisdirectedRequest = 421,
    UnprocessableContent = 422,
    UpgradeRequired = 426,

    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
}

impl Display for HttpStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpStatus::Continue => write!(formatter, "100 Continue"),
            HttpStatus::SwitchingProtocols => write!(formatter, "101 Switching Protocols"),
            
            HttpStatus::OK => write!(formatter, "200 OK"),
            HttpStatus::Created => write!(formatter, "201 Created"),
            HttpStatus::Accepted => write!(formatter, "202 Accepted"),
            HttpStatus::NonAuthoritativeInformation => write!(formatter, "203 Non-Authoritative Information"),
            HttpStatus::NoContent => write!(formatter, "204 No Content"),
            HttpStatus::ResetContent => write!(formatter, "205 Reset Content"),
            HttpStatus::PartialContent => write!(formatter, "206 Partial Content"),
            
            HttpStatus::MultipleChoices => write!(formatter, "300 Multiple Choices"),
            HttpStatus::MovedPermanently => write!(formatter, "301 Moved Permanently"),
            HttpStatus::Found => write!(formatter, "302 Found"),
            HttpStatus::SeeOther => write!(formatter, "303 See Other"),
            HttpStatus::NotModified => write!(formatter, "304 Not Modified"),
            HttpStatus::UseProxy => write!(formatter, "305 Use Proxy"),
            HttpStatus::TemporaryRedirect => write!(formatter, "307 Temporary Redirect"),
            HttpStatus::PermanentRedirect => write!(formatter, "308 Permanent Redirect"),
            
            HttpStatus::BadRequest => write!(formatter, "400 Bad Request"),
            HttpStatus::Unauthorized => write!(formatter, "401 Unauthorized"),
            HttpStatus::PaymentRequired => write!(formatter, "402 Payment Required"),
            HttpStatus::Forbidden => write!(formatter, "403 Forbidden"),
            HttpStatus::NotFound => write!(formatter, "404 Not Found"),
            HttpStatus::MethodNotAllowed => write!(formatter, "405 Method Not Allowed"),
            HttpStatus::NotAcceptable => write!(formatter, "406 Not Acceptable"),
            HttpStatus::ProxyAuthenticationRequired => write!(formatter, "407 Proxy Authentication Required"),
            HttpStatus::RequestTimeout => write!(formatter, "408 Request Timeout"),
            HttpStatus::Conflict => write!(formatter, "409 Conflict"),
            HttpStatus::Gone => write!(formatter, "410 Gone"),
            HttpStatus::LengthRequired => write!(formatter, "411 Length Required"),
            HttpStatus::PreconditionFailed => write!(formatter, "412 Precondition Failed"),
            HttpStatus::ContentTooLarge => write!(formatter, "413 Content Too Large"),
            HttpStatus::UriTooLong => write!(formatter, "414 Uri Too Long"),
            HttpStatus::UnsupportedMediaType => write!(formatter, "415 Unsupported Media Type"),
            HttpStatus::RangeNotSatisfiable => write!(formatter, "416 Range Not Satisfiable"),
            HttpStatus::ExpectationFailed => write!(formatter, "417 Expectation Failed"),
            HttpStatus::MisdirectedRequest => write!(formatter, "421 Misdirected Request"),
            HttpStatus::UnprocessableContent => write!(formatter, "422 Unprocessable Content"),
            HttpStatus::UpgradeRequired => write!(formatter, "426 Upgrade Required"),
            
            HttpStatus::InternalServerError => write!(formatter, "500 Internal Server Error"),
            HttpStatus::NotImplemented => write!(formatter, "501 Not Implemented"),
            HttpStatus::BadGateway => write!(formatter, "502 Bad Gateway"),
            HttpStatus::ServiceUnavailable => write!(formatter, "503 Service Unavailable"),
            HttpStatus::GatewayTimeout => write!(formatter, "504 Gateway Timeout"),
            HttpStatus::HttpVersionNotSupported => write!(formatter, "505 HTTP Version Not Supported"),
        }
    }
}