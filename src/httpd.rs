#![allow(non_camel_case_types)]

use libc::{c_void, c_int, c_char};

use std::ffi::CString;
use ffi;
use std::fmt;

use wrapper::{Wrapper, from_char_ptr};

use apr::AprTable;
use cookie::Cookie;


pub enum Status {
   // non-HTTP status codes returned by hooks
   OK,            // Module has handled this stage.
   DECLINED,      // Module declines to handle
   DONE,          // Module has served the response completely
                  // - it's safe to die() with no more output
   SUSPENDED,     // Module will handle the remainder of the request.
                  // The core will never invoke the request again,

   HTTP_CONTINUE,
   HTTP_SWITCHING_PROTOCOLS,
   HTTP_PROCESSING,
   HTTP_OK,
   HTTP_CREATED,
   HTTP_ACCEPTED,
   HTTP_NON_AUTHORITATIVE,
   HTTP_NO_CONTENT,
   HTTP_RESET_CONTENT,
   HTTP_PARTIAL_CONTENT,
   HTTP_MULTI_STATUS,
   HTTP_ALREADY_REPORTED,
   HTTP_IM_USED,
   HTTP_MULTIPLE_CHOICES,
   HTTP_MOVED_PERMANENTLY,
   HTTP_MOVED_TEMPORARILY,
   HTTP_SEE_OTHER,
   HTTP_NOT_MODIFIED,
   HTTP_USE_PROXY,
   HTTP_TEMPORARY_REDIRECT,
   HTTP_PERMANENT_REDIRECT,
   HTTP_BAD_REQUEST,
   HTTP_UNAUTHORIZED,
   HTTP_PAYMENT_REQUIRED,
   HTTP_FORBIDDEN,
   HTTP_NOT_FOUND,
   HTTP_METHOD_NOT_ALLOWED,
   HTTP_NOT_ACCEPTABLE,
   HTTP_PROXY_AUTHENTICATION_REQUIRED,
   HTTP_REQUEST_TIME_OUT,
   HTTP_CONFLICT,
   HTTP_GONE,
   HTTP_LENGTH_REQUIRED,
   HTTP_PRECONDITION_FAILED,
   HTTP_REQUEST_ENTITY_TOO_LARGE,
   HTTP_REQUEST_URI_TOO_LARGE,
   HTTP_UNSUPPORTED_MEDIA_TYPE,
   HTTP_RANGE_NOT_SATISFIABLE,
   HTTP_EXPECTATION_FAILED,
   HTTP_IM_A_TEAPOT,
   HTTP_UNPROCESSABLE_ENTITY,
   HTTP_LOCKED,
   HTTP_FAILED_DEPENDENCY,
   HTTP_UPGRADE_REQUIRED,
   HTTP_PRECONDITION_REQUIRED,
   HTTP_TOO_MANY_REQUESTS,
   HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE,
   HTTP_INTERNAL_SERVER_ERROR,
   HTTP_NOT_IMPLEMENTED,
   HTTP_BAD_GATEWAY,
   HTTP_SERVICE_UNAVAILABLE,
   HTTP_GATEWAY_TIME_OUT,
   HTTP_VERSION_NOT_SUPPORTED,
   HTTP_VARIANT_ALSO_VARIES,
   HTTP_INSUFFICIENT_STORAGE,
   HTTP_LOOP_DETECTED,
   HTTP_NOT_EXTENDED,
   HTTP_NETWORK_AUTHENTICATION_REQUIRED,
}

impl Into<c_int> for Status {
   fn into(self) -> c_int {
      match self {
         Status::OK => ffi::OK,
         Status::DECLINED => ffi::DECLINED,
         Status::DONE => ffi::DONE,
         Status::SUSPENDED => ffi::SUSPENDED,

         Status::HTTP_CONTINUE => ffi::HTTP_CONTINUE,
         Status::HTTP_SWITCHING_PROTOCOLS => ffi::HTTP_SWITCHING_PROTOCOLS,
         Status::HTTP_PROCESSING => ffi::HTTP_PROCESSING,
         Status::HTTP_OK => ffi::HTTP_OK,
         Status::HTTP_CREATED => ffi::HTTP_CREATED,
         Status::HTTP_ACCEPTED => ffi::HTTP_ACCEPTED,
         Status::HTTP_NON_AUTHORITATIVE => ffi::HTTP_NON_AUTHORITATIVE,
         Status::HTTP_NO_CONTENT => ffi::HTTP_NO_CONTENT,
         Status::HTTP_RESET_CONTENT => ffi::HTTP_RESET_CONTENT,
         Status::HTTP_PARTIAL_CONTENT => ffi::HTTP_PARTIAL_CONTENT,
         Status::HTTP_MULTI_STATUS => ffi::HTTP_MULTI_STATUS,
         Status::HTTP_ALREADY_REPORTED => ffi::HTTP_ALREADY_REPORTED,
         Status::HTTP_IM_USED => ffi::HTTP_IM_USED,
         Status::HTTP_MULTIPLE_CHOICES => ffi::HTTP_MULTIPLE_CHOICES,
         Status::HTTP_MOVED_PERMANENTLY => ffi::HTTP_MOVED_PERMANENTLY,
         Status::HTTP_MOVED_TEMPORARILY => ffi::HTTP_MOVED_TEMPORARILY,
         Status::HTTP_SEE_OTHER => ffi::HTTP_SEE_OTHER,
         Status::HTTP_NOT_MODIFIED => ffi::HTTP_NOT_MODIFIED,
         Status::HTTP_USE_PROXY => ffi::HTTP_USE_PROXY,
         Status::HTTP_TEMPORARY_REDIRECT => ffi::HTTP_TEMPORARY_REDIRECT,
         Status::HTTP_PERMANENT_REDIRECT => ffi::HTTP_PERMANENT_REDIRECT,
         Status::HTTP_BAD_REQUEST => ffi::HTTP_BAD_REQUEST,
         Status::HTTP_UNAUTHORIZED => ffi::HTTP_UNAUTHORIZED,
         Status::HTTP_PAYMENT_REQUIRED => ffi::HTTP_PAYMENT_REQUIRED,
         Status::HTTP_FORBIDDEN => ffi::HTTP_FORBIDDEN,
         Status::HTTP_NOT_FOUND => ffi::HTTP_NOT_FOUND,
         Status::HTTP_METHOD_NOT_ALLOWED => ffi::HTTP_METHOD_NOT_ALLOWED,
         Status::HTTP_NOT_ACCEPTABLE => ffi::HTTP_NOT_ACCEPTABLE,
         Status::HTTP_PROXY_AUTHENTICATION_REQUIRED => ffi::HTTP_PROXY_AUTHENTICATION_REQUIRED,
         Status::HTTP_REQUEST_TIME_OUT => ffi::HTTP_REQUEST_TIME_OUT,
         Status::HTTP_CONFLICT => ffi::HTTP_CONFLICT,
         Status::HTTP_GONE => ffi::HTTP_GONE,
         Status::HTTP_LENGTH_REQUIRED => ffi::HTTP_LENGTH_REQUIRED,
         Status::HTTP_PRECONDITION_FAILED => ffi::HTTP_PRECONDITION_FAILED,
         Status::HTTP_REQUEST_ENTITY_TOO_LARGE => ffi::HTTP_REQUEST_ENTITY_TOO_LARGE,
         Status::HTTP_REQUEST_URI_TOO_LARGE => ffi::HTTP_REQUEST_URI_TOO_LARGE,
         Status::HTTP_UNSUPPORTED_MEDIA_TYPE => ffi::HTTP_UNSUPPORTED_MEDIA_TYPE,
         Status::HTTP_RANGE_NOT_SATISFIABLE => ffi::HTTP_RANGE_NOT_SATISFIABLE,
         Status::HTTP_EXPECTATION_FAILED => ffi::HTTP_EXPECTATION_FAILED,
         Status::HTTP_IM_A_TEAPOT => ffi::HTTP_IM_A_TEAPOT,
         Status::HTTP_UNPROCESSABLE_ENTITY => ffi::HTTP_UNPROCESSABLE_ENTITY,
         Status::HTTP_LOCKED => ffi::HTTP_LOCKED,
         Status::HTTP_FAILED_DEPENDENCY => ffi::HTTP_FAILED_DEPENDENCY,
         Status::HTTP_UPGRADE_REQUIRED => ffi::HTTP_UPGRADE_REQUIRED,
         Status::HTTP_PRECONDITION_REQUIRED => ffi::HTTP_PRECONDITION_REQUIRED,
         Status::HTTP_TOO_MANY_REQUESTS => ffi::HTTP_TOO_MANY_REQUESTS,
         Status::HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE => ffi::HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE,
         Status::HTTP_INTERNAL_SERVER_ERROR => ffi::HTTP_INTERNAL_SERVER_ERROR,
         Status::HTTP_NOT_IMPLEMENTED => ffi::HTTP_NOT_IMPLEMENTED,
         Status::HTTP_BAD_GATEWAY => ffi::HTTP_BAD_GATEWAY,
         Status::HTTP_SERVICE_UNAVAILABLE => ffi::HTTP_SERVICE_UNAVAILABLE,
         Status::HTTP_GATEWAY_TIME_OUT => ffi::HTTP_GATEWAY_TIME_OUT,
         Status::HTTP_VERSION_NOT_SUPPORTED => ffi::HTTP_VERSION_NOT_SUPPORTED,
         Status::HTTP_VARIANT_ALSO_VARIES => ffi::HTTP_VARIANT_ALSO_VARIES,
         Status::HTTP_INSUFFICIENT_STORAGE => ffi::HTTP_INSUFFICIENT_STORAGE,
         Status::HTTP_LOOP_DETECTED => ffi::HTTP_LOOP_DETECTED,
         Status::HTTP_NOT_EXTENDED => ffi::HTTP_NOT_EXTENDED,
         Status::HTTP_NETWORK_AUTHENTICATION_REQUIRED => ffi::HTTP_NETWORK_AUTHENTICATION_REQUIRED,
      }
   }
}



impl Into<Status> for c_int {
   fn into(self) -> Status {
      match self {
         ffi::OK => Status::OK,
         ffi::DECLINED => Status::DECLINED,
         ffi::DONE => Status::DONE,
         ffi::SUSPENDED => Status::SUSPENDED,

         ffi::HTTP_CONTINUE => Status::HTTP_CONTINUE,
         ffi::HTTP_SWITCHING_PROTOCOLS => Status::HTTP_SWITCHING_PROTOCOLS,
         ffi::HTTP_PROCESSING => Status::HTTP_PROCESSING,
         ffi::HTTP_OK => Status::HTTP_OK,
         ffi::HTTP_CREATED => Status::HTTP_CREATED,
         ffi::HTTP_ACCEPTED => Status::HTTP_ACCEPTED,
         ffi::HTTP_NON_AUTHORITATIVE => Status::HTTP_NON_AUTHORITATIVE,
         ffi::HTTP_NO_CONTENT => Status::HTTP_NO_CONTENT,
         ffi::HTTP_RESET_CONTENT => Status::HTTP_RESET_CONTENT,
         ffi::HTTP_PARTIAL_CONTENT => Status::HTTP_PARTIAL_CONTENT,
         ffi::HTTP_MULTI_STATUS => Status::HTTP_MULTI_STATUS,
         ffi::HTTP_ALREADY_REPORTED => Status::HTTP_ALREADY_REPORTED,
         ffi::HTTP_IM_USED => Status::HTTP_IM_USED,
         ffi::HTTP_MULTIPLE_CHOICES => Status::HTTP_MULTIPLE_CHOICES,
         ffi::HTTP_MOVED_PERMANENTLY => Status::HTTP_MOVED_PERMANENTLY,
         ffi::HTTP_MOVED_TEMPORARILY => Status::HTTP_MOVED_TEMPORARILY,
         ffi::HTTP_SEE_OTHER => Status::HTTP_SEE_OTHER,
         ffi::HTTP_NOT_MODIFIED => Status::HTTP_NOT_MODIFIED,
         ffi::HTTP_USE_PROXY => Status::HTTP_USE_PROXY,
         ffi::HTTP_TEMPORARY_REDIRECT => Status::HTTP_TEMPORARY_REDIRECT,
         ffi::HTTP_PERMANENT_REDIRECT => Status::HTTP_PERMANENT_REDIRECT,
         ffi::HTTP_BAD_REQUEST => Status::HTTP_BAD_REQUEST,
         ffi::HTTP_UNAUTHORIZED => Status::HTTP_UNAUTHORIZED,
         ffi::HTTP_PAYMENT_REQUIRED => Status::HTTP_PAYMENT_REQUIRED,
         ffi::HTTP_FORBIDDEN => Status::HTTP_FORBIDDEN,
         ffi::HTTP_NOT_FOUND => Status::HTTP_NOT_FOUND,
         ffi::HTTP_METHOD_NOT_ALLOWED => Status::HTTP_METHOD_NOT_ALLOWED,
         ffi::HTTP_NOT_ACCEPTABLE => Status::HTTP_NOT_ACCEPTABLE,
         ffi::HTTP_PROXY_AUTHENTICATION_REQUIRED => Status::HTTP_PROXY_AUTHENTICATION_REQUIRED,
         ffi::HTTP_REQUEST_TIME_OUT => Status::HTTP_REQUEST_TIME_OUT,
         ffi::HTTP_CONFLICT => Status::HTTP_CONFLICT,
         ffi::HTTP_GONE => Status::HTTP_GONE,
         ffi::HTTP_LENGTH_REQUIRED => Status::HTTP_LENGTH_REQUIRED,
         ffi::HTTP_PRECONDITION_FAILED => Status::HTTP_PRECONDITION_FAILED,
         ffi::HTTP_REQUEST_ENTITY_TOO_LARGE => Status::HTTP_REQUEST_ENTITY_TOO_LARGE,
         ffi::HTTP_REQUEST_URI_TOO_LARGE => Status::HTTP_REQUEST_URI_TOO_LARGE,
         ffi::HTTP_UNSUPPORTED_MEDIA_TYPE => Status::HTTP_UNSUPPORTED_MEDIA_TYPE,
         ffi::HTTP_RANGE_NOT_SATISFIABLE => Status::HTTP_RANGE_NOT_SATISFIABLE,
         ffi::HTTP_EXPECTATION_FAILED => Status::HTTP_EXPECTATION_FAILED,
         ffi::HTTP_IM_A_TEAPOT => Status::HTTP_IM_A_TEAPOT,
         ffi::HTTP_UNPROCESSABLE_ENTITY => Status::HTTP_UNPROCESSABLE_ENTITY,
         ffi::HTTP_LOCKED => Status::HTTP_LOCKED,
         ffi::HTTP_FAILED_DEPENDENCY => Status::HTTP_FAILED_DEPENDENCY,
         ffi::HTTP_UPGRADE_REQUIRED => Status::HTTP_UPGRADE_REQUIRED,
         ffi::HTTP_PRECONDITION_REQUIRED => Status::HTTP_PRECONDITION_REQUIRED,
         ffi::HTTP_TOO_MANY_REQUESTS => Status::HTTP_TOO_MANY_REQUESTS,
         ffi::HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE => Status::HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE,
         ffi::HTTP_INTERNAL_SERVER_ERROR => Status::HTTP_INTERNAL_SERVER_ERROR,
         ffi::HTTP_NOT_IMPLEMENTED => Status::HTTP_NOT_IMPLEMENTED,
         ffi::HTTP_BAD_GATEWAY => Status::HTTP_BAD_GATEWAY,
         ffi::HTTP_SERVICE_UNAVAILABLE => Status::HTTP_SERVICE_UNAVAILABLE,
         ffi::HTTP_GATEWAY_TIME_OUT => Status::HTTP_GATEWAY_TIME_OUT,
         ffi::HTTP_VERSION_NOT_SUPPORTED => Status::HTTP_VERSION_NOT_SUPPORTED,
         ffi::HTTP_VARIANT_ALSO_VARIES => Status::HTTP_VARIANT_ALSO_VARIES,
         ffi::HTTP_INSUFFICIENT_STORAGE => Status::HTTP_INSUFFICIENT_STORAGE,
         ffi::HTTP_LOOP_DETECTED => Status::HTTP_LOOP_DETECTED,
         ffi::HTTP_NOT_EXTENDED => Status::HTTP_NOT_EXTENDED,
         ffi::HTTP_NETWORK_AUTHENTICATION_REQUIRED => Status::HTTP_NETWORK_AUTHENTICATION_REQUIRED,

         _ => Status::DECLINED
      }
   }
}

pub type StatusResult = Result<Status, &'static str>;

pub enum ProxyReq {
   NONE,     // No proxy
   PROXY,    // Standard proxy
   REVERSE,  // Reverse proxy
   RESPONSE, // Origin response
}

impl Into<ProxyReq> for c_int {
   fn into(self) -> ProxyReq {
      match self {
         ffi::PROXYREQ_NONE => ProxyReq::NONE,
         ffi::PROXYREQ_PROXY => ProxyReq::PROXY,
         ffi::PROXYREQ_REVERSE => ProxyReq::REVERSE,
         ffi::PROXYREQ_RESPONSE => ProxyReq::RESPONSE,
         _ => panic!("Unknown ProxyReq type")
      }
   }
}

impl fmt::Display for ProxyReq {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let display = match *self {
         ProxyReq::NONE => "No proxy",
         ProxyReq::PROXY => "Standard proxy",
         ProxyReq::REVERSE => "Reverse proxy",
         ProxyReq::RESPONSE => "Origin response"
      };

      write!(f, "{}", display)
   }
}


pub type Request<'a> = Wrapper<'a, ffi::request_rec>;


impl<'a> Request<'a> {
   pub fn connection(&self) -> Result<Conn, &'static str> {
      Wrapper::from_raw_ptr(self.raw.connection)
   }

   pub fn the_request(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.the_request)
   }

   pub fn http09(&self) -> bool {
      self.raw.assbackwards != 0
   }

   pub fn proxyreq(&self) -> ProxyReq {
      self.raw.proxyreq.into()
   }

   pub fn header_only(&self) -> bool {
      self.raw.header_only != 0
   }

   pub fn set_header_only(&mut self, header_only: bool) {
      self.raw.header_only = header_only as c_int;
   }

   pub fn protocol(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.protocol)
   }

   pub fn hostname(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.hostname)
   }

   pub fn request_time(&self) -> i64 {
      self.raw.request_time
   }

   pub fn status_line(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.status_line)
   }

   pub fn status(&self) -> Status {
      self.raw.status.into()
   }

   pub fn set_status(&mut self, status: Status) {
      self.raw.status = status.into();
   }

   pub fn method(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.method)
   }

   pub fn mtime(&self) -> i64 {
      self.raw.mtime
   }

   pub fn set_last_modified(&mut self, mtime: i64) {
      unsafe {
         ffi::ap_update_mtime(self.raw, mtime);
         ffi::ap_set_last_modified(self.raw);
      }
   }

   pub fn range(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.range)
   }

   pub fn clength(&self) -> i64 {
      return self.raw.clength
   }

   pub fn headers_in(&self) -> Result<AprTable, &'static str> {
      Wrapper::from_raw_ptr(self.raw.headers_in)
   }

   pub fn headers_out(&self) -> Result<AprTable, &'static str> {
      Wrapper::from_raw_ptr(self.raw.headers_out)
   }

   pub fn err_headers_out(&self) -> Result<AprTable, &'static str> {
      Wrapper::from_raw_ptr(self.raw.err_headers_out)
   }

   pub fn subprocess_env(&self) -> Result<AprTable, &'static str> {
      Wrapper::from_raw_ptr(self.raw.subprocess_env)
   }

   pub fn notes(&self) -> Result<AprTable, &'static str> {
      Wrapper::from_raw_ptr(self.raw.notes)
   }

   pub fn content_type(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.content_type)
   }

   pub fn set_content_type<T: Into<Vec<u8>>>(&self, ct: T) {
      let c_str = ffi::dup_c_str(self.raw.pool, ct);

      unsafe {
         ffi::ap_set_content_type(
            self.raw,
            c_str
         );
      };
   }

   pub fn handler(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.handler)
   }

   pub fn content_encoding(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.content_encoding)
   }

   pub fn vlist_validator(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.vlist_validator)
   }

   pub fn user(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.user)
   }

   pub fn auth_type(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.ap_auth_type)
   }

   pub fn unparsed_uri(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.unparsed_uri)
   }

   pub fn uri(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.uri)
   }

   pub fn filename(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.filename)
   }

   pub fn canonical_filename(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.canonical_filename)
   }

   pub fn path_info(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.path_info)
   }

   pub fn args(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.args)
   }

   pub fn log_id(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.log_id)
   }

   pub fn useragent_ip(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.useragent_ip)
   }

   pub fn write<T: Into<Vec<u8>>>(&self, data: T) {
      let c_str_buf = CString::new(data).unwrap();

      unsafe {
         ffi::ap_rwrite(
            c_str_buf.as_ptr() as *mut c_void,
            c_str_buf.to_bytes().len() as i32,
            self.raw
         );
      }
   }

   pub fn escape_html<T: Into<Vec<u8>>>(&self, s: T) -> Result<&'a str, &'static str> {
      let c_str = CString::new(s).unwrap();

      let escaped = unsafe {
         ffi::ap_escape_html2(
            self.raw.pool,
            c_str.as_ptr(),
            0
         )
      };

      from_char_ptr(escaped)
   }

   pub fn escape_urlencoded<T: Into<Vec<u8>>>(&self, s: T) -> Result<&'a str, &'static str> {
      let c_str = CString::new(s).unwrap();

      let escaped = unsafe {
         ffi::ap_escape_urlencoded(self.raw.pool, c_str.as_ptr())
      };

      from_char_ptr(escaped)
   }

   pub fn unescape_urlencoded<T: Into<Vec<u8>>>(&self, query: T) -> Result<&'a str, &'static str> {
      let c_str = ffi::dup_c_str(self.raw.pool, query);

      let res = unsafe {
         ffi::ap_unescape_urlencoded(c_str)
      };

      if res != 0 {
         return Err("Unescape URL-encoded error");
      };

      from_char_ptr(c_str)
   }

   pub fn server_name(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(
         unsafe { ffi::ap_get_server_name(self.raw) }
      )
   }

   pub fn server_port(&self) -> u16 {
      unsafe { ffi::ap_get_server_port(self.raw) }
   }

   pub fn document_root(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(
         unsafe { ffi::ap_document_root(self.raw) }
      )
   }

   pub fn auth_name(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(
         unsafe { ffi::ap_auth_name(self.raw) }
      )
   }

   pub fn basic_auth_pw(&self) -> Result<&'a str, &'static str> {
      let mut pw: *const c_char = ::std::ptr::null_mut();

      unsafe {
         ffi::ap_get_basic_auth_pw(self.raw, &mut pw);
      }

      from_char_ptr(pw)

   }

   pub fn context_document_root(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(
         unsafe { ffi::ap_context_document_root(self.raw) }
      )
   }

   pub fn context_prefix(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(
         unsafe { ffi::ap_context_prefix(self.raw) }
      )
   }

   pub fn http_scheme(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(
         unsafe { ffi::ap_run_http_scheme(self.raw) }
      )
   }

   pub fn default_port(&self) -> u16 {
      unsafe { ffi::ap_run_default_port(self.raw) }
   }

   pub fn is_initial_req(&self) -> bool {
      unsafe { ffi::ap_is_initial_req(self.raw) == 1 }
   }

   pub fn some_auth_required(&self) -> bool {
      unsafe { ffi::ap_some_auth_required(self.raw) == 1 }
   }

   pub fn cookie<T: Into<Vec<u8>>>(&self, name: T) -> Result<&'a str, &'static str> {
      let c_str_name = ffi::dup_c_str(self.raw.pool, name);
      let mut val: *const c_char = ::std::ptr::null_mut();

      unsafe {
         ffi::ap_cookie_read(self.raw, c_str_name, &mut val, 0);
      }

      from_char_ptr(val)
   }

   pub fn set_cookie(&self, cookie: Cookie) {
      let c_str_name = ffi::dup_c_str(self.raw.pool, cookie.name);
      let c_str_val = ffi::dup_c_str(self.raw.pool, cookie.value);
      let c_str_attrs = ffi::dup_c_str(self.raw.pool, cookie.attrs(&self));

      let null: *const ffi::apr_table_t = ::std::ptr::null();

      unsafe {
         ffi::ap_cookie_write(self.raw, c_str_name, c_str_val, c_str_attrs, 0,
            self.raw.headers_out, null);
      }
   }

   pub fn base64_encode<T: Into<Vec<u8>>>(&self, plain: T) -> Result<&'a str, &'static str> {
      let c_str_plain: CString = match CString::new(plain) {
         Ok(val) => val,
         Err(_) => return Err("Error allocating CString from plain argument")
      };

      let plain_len: c_int = c_str_plain.to_bytes().len() as c_int;

      let mut encoded_len: c_int = unsafe {
         ffi::apr_base64_encode_len(plain_len)
      };

      if encoded_len == 0 {
         return Err("Base64 encode len error");
      };

      let encoded: *mut c_char = unsafe {
         ffi::apr_palloc(self.raw.pool, encoded_len as ffi::apr_size_t) as *mut c_char
      };

      encoded_len = unsafe {
         ffi::apr_base64_encode(encoded, c_str_plain.as_ptr(), plain_len)
      };

      if encoded_len == 0 {
         return Err("Base64 encode error");
      };

      from_char_ptr(encoded)
   }

   pub fn base64_decode<T: Into<Vec<u8>>>(&self, encoded: T) -> Result<&'a str, &'static str> {
      let c_str_encoded: CString = match CString::new(encoded) {
         Ok(val) => val,
         Err(_) => return Err("Error allocating CString from Base64 encoded argument")
      };

      let mut plain_len: c_int = unsafe {
         ffi::apr_base64_decode_len(c_str_encoded.as_ptr())
      };

      if plain_len == 0 {
         return Err("Base64 decode len error");
      };

      let plain: *mut c_char = unsafe {
         ffi::apr_palloc(self.raw.pool, plain_len as ffi::apr_size_t) as *mut c_char
      };

      plain_len = unsafe {
         ffi::apr_base64_decode(plain, c_str_encoded.as_ptr())
      };

      if plain_len == 0 {
         return Err("Base64 decode error");
      };

      from_char_ptr(plain)
   }

   pub fn rfc822_date(&self, t: i64) -> Result<&'a str, &'static str> {
      let date: *mut c_char = unsafe {
         ffi::apr_palloc(self.raw.pool, ffi::APR_RFC822_DATE_LEN) as *mut c_char
      };

      unsafe {
         ffi::apr_rfc822_date(date, t);
      }

      from_char_ptr(date)
   }
}

pub type Conn<'a> = Wrapper<'a, ffi::conn_rec>;


impl<'a> Conn<'a> {
   pub fn client_ip(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.client_ip)
   }

   pub fn remote_host(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.remote_host)
   }

   pub fn remote_logname(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.remote_logname)
   }

   pub fn local_ip(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.local_ip)
   }

   pub fn local_host(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.local_host)
   }

   pub fn log_id(&self) -> Result<&'a str, &'static str> {
      from_char_ptr(self.raw.log_id)
   }
}


pub fn server_banner<'a>() -> Result<&'a str, &'static str> {
   from_char_ptr(
      unsafe { ffi::ap_get_server_banner() }
   )
}

pub fn server_description<'a>() -> Result<&'a str, &'static str> {
   from_char_ptr(
      unsafe { ffi::ap_get_server_description() }
   )
}

pub fn server_built<'a>() -> Result<&'a str, &'static str> {
   from_char_ptr(
      unsafe { ffi::ap_get_server_built() }
   )
}

pub fn show_mpm<'a>() -> Result<&'a str, &'static str> {
   from_char_ptr(
      unsafe { ffi::ap_show_mpm() }
   )
}
