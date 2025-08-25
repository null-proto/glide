// Response
pub const DATE: &'static str = "Date";
pub const CONNECTION: &'static str = "Connection";

// Representation Data and Metadata
pub const HOST: &'static str = "Host";
pub const CONTENT_TYPE: &'static str = "Content-Type";
pub const CONTENT_LENGTH: &'static str = "Content-Length";
pub const CONTENT_LANGUAGE: &'static str = "Content-Language";
pub const CONTENT_LOCATION: &'static str = "Content-Location";
pub const LAST_MODIFIED: &'static str = "Last-Modified";
pub const ETAG: &'static str = "ETag";

// Message Context
pub const EXPECT: &'static str = "Expect";
pub const FROM: &'static str = "From";
pub const REFERER: &'static str = "Referer";
pub const TE: &'static str = "TE";
pub const USER_AGENT: &'static str = "User-Agent";

// Response Context Fields
pub const ALLOW: &'static str = "Allow";
pub const LOCATION: &'static str = "Location";
pub const RETRY_AFTER: &'static str = "Retry-After";
pub const SERVER: &'static str = "Server";

// HTTP Authentication
pub const WWW_AUTHENTICATE: &'static str = "WWW-Authenticate";
pub const AUTHORIZATION: &'static str = "Authorization";
pub const AUTHENTICATION_INFO: &'static str = "Authentication-Info";
// Authentication Clients to Proxy
pub const PROXY_AUTHENTICATE: &'static str = "Proxy-Authenticate";
pub const PROXY_AUTHENTICATION_INFO: &'static str = "Proxy-Authentication-Info";

// Content Negotiation Fields
pub const ACCEPT: &'static str = "Accept";
pub const ACCEPT_CHARSET: &'static str = "Accept-Charset";
pub const ACCEPT_ENCODING: &'static str = "Accept-Encoding";
pub const ACCEPT_LANGUAGE: &'static str = "Accept-Language";
pub const VARY: &'static str = "Vary";

// Conditional Requests
pub const IF_MATCH: &'static str = "If-Match";
pub const IF_NONE_MATCH: &'static str = "If-None-Match";
pub const IF_MODIFIED_SINCE: &'static str = "If-Modified-Since";
pub const IF_UNMODIFIED_SINCE: &'static str = "If-Unmodified-Since";
pub const IF_RANGE: &'static str = "If-Range";

// Range
pub const RANGE: &'static str = "Range";
pub const ACCEPT_RANGES: &'static str = "Accept-Ranges";
pub const CONTENT_RANGE: &'static str = "Content-Range";
