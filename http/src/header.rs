#![allow(unused)]
// Representation Data and Metadata 
const HOST : &'static str = "Host";
const CONTENT_TYPE : &'static str = "Content-Type";
const CONTENT_LENGTH : &'static str = "Content-Length";
const CONTENT_LANGUAGE : &'static str = "Content-Language";
const CONTENT_LOCATION : &'static str = "Content-Location";
const LAST_MODIFIED : &'static str = "Last-Modified";
const ETAG : &'static str = "ETag";

// Message Context
const EXPECT : &'static str = "Expect";
const FROM : &'static str = "From";
const REFERER : &'static str = "Referer";
const TE : &'static str = "TE";
const USER_AGENT : &'static str = "User-Agent";

// Response Context Fields
const ALLOW : &'static str = "Allow";
const LOCATION : &'static str = "Location";
const RETRY_AFTER : &'static str = "Retry-After";
const SERVER : &'static str = "Server";

// HTTP Authentication
const WWW_AUTHENTICATE : &'static str = "WWW-Authenticate";
const AUTHORIZATION : &'static str = "Authorization";
const AUTHENTICATION_INFO : &'static str = "Authentication-Info";

// Authentication Clients to Proxy
const PROXY_AUTHENTICATE : &'static str = "Proxy-Authenticate";
const PROXY_AUTHENTICATION_INFO : &'static str = "Proxy-Authentication-Info";

// Content Negotiation Fields
const ACCEPT : &'static str = "Accept";
const ACCEPT_CHARSET : &'static str = "Accept-Charset";
const ACCEPT_ENCODING : &'static str = "Accept-Encoding";
const ACCEPT_LANGUAGE : &'static str = "Accept-Language";
const VARY : &'static str = "Vary";

// Conditional Requests
const IF_MATCH : &'static str = "If-Match";
const IF_NONE_MATCH : &'static str = "If-None-Match";
const IF_MODIFIED_SINCE : &'static str = "If-Modified-Since";
const IF_UNMODIFIED_SINCE : &'static str = "If-Unmodified-Since";
const IF_RANGE : &'static str = "If-Range";

// Range
const RANGE : &'static str = "Range";
const ACCEPT_RANGES : &'static str = "Accept-Ranges";
const CONTENT_RANGE : &'static str = "Content-Range";
