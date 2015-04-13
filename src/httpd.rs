pub mod raw {
}


use libc::{c_void, c_int, c_char};

use std::ffi::CString;
use ffi;
use std::fmt;

use wrapper::{Wrapper, c_str_value, wrap_ptr};

use apr::AprTable;


pub enum Status {
   // non-HTTP status codes returned by hooks
   OK,            // Module has handled this stage.
   DECLINED,      // Module declines to handle
   DONE,          // Module has served the response completely
                  // - it's safe to die() with no more output
   SUSPENDED,     // Module will handle the remainder of the request.
                  // The core will never invoke the request again,
}

impl Into<c_int> for Status {
   fn into(self) -> c_int {
      match self {
         Status::OK => ffi::OK,
         Status::DECLINED => ffi::DECLINED,
         Status::DONE => ffi::DONE,
         Status::SUSPENDED => ffi::SUSPENDED,
      }
   }
}

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
   pub fn connection(&self) -> Option<Conn> {
      wrap_ptr(self.raw.connection)
   }

   pub fn the_request(&self) -> Option<&'a str> {
      c_str_value(self.raw.the_request)
   }

   pub fn http09(&self) -> bool {
      self.raw.assbackwards != 0
   }

   pub fn proxyreq(&self) -> ProxyReq {
      self.raw.proxyreq.into()
   }

   pub fn protocol(&self) -> Option<&'a str> {
      c_str_value(self.raw.protocol)
   }

   pub fn hostname(&self) -> Option<&'a str> {
      c_str_value(self.raw.hostname)
   }

   pub fn status_line(&self) -> Option<&'a str> {
      c_str_value(self.raw.status_line)
   }

   pub fn method(&self) -> Option<&'a str> {
      c_str_value(self.raw.method)
   }

   pub fn range(&self) -> Option<&'a str> {
      c_str_value(self.raw.range)
   }

   pub fn clength(&self) -> i64 {
      return self.raw.clength
   }

   pub fn headers_in(&self) -> Option<AprTable> {
      wrap_ptr(self.raw.headers_in)
   }

   pub fn headers_out(&self) -> Option<AprTable> {
      wrap_ptr(self.raw.headers_out)
   }

   pub fn err_headers_out(&self) -> Option<AprTable> {
      wrap_ptr(self.raw.err_headers_out)
   }

   pub fn subprocess_env(&self) -> Option<AprTable> {
      wrap_ptr(self.raw.subprocess_env)
   }

   pub fn notes(&self) -> Option<AprTable> {
      wrap_ptr(self.raw.notes)
   }

   pub fn content_type(&self) -> Option<&'a str> {
      c_str_value(self.raw.content_type)
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

   pub fn handler(&self) -> Option<&'a str> {
      c_str_value(self.raw.handler)
   }

   pub fn content_encoding(&self) -> Option<&'a str> {
      c_str_value(self.raw.content_encoding)
   }

   pub fn vlist_validator(&self) -> Option<&'a str> {
      c_str_value(self.raw.vlist_validator)
   }

   pub fn user(&self) -> Option<&'a str> {
      c_str_value(self.raw.user)
   }

   pub fn auth_type(&self) -> Option<&'a str> {
      c_str_value(self.raw.ap_auth_type)
   }

   pub fn unparsed_uri(&self) -> Option<&'a str> {
      c_str_value(self.raw.unparsed_uri)
   }

   pub fn uri(&self) -> Option<&'a str> {
      c_str_value(self.raw.uri)
   }

   pub fn filename(&self) -> Option<&'a str> {
      c_str_value(self.raw.filename)
   }

   pub fn canonical_filename(&self) -> Option<&'a str> {
      c_str_value(self.raw.canonical_filename)
   }

   pub fn path_info(&self) -> Option<&'a str> {
      c_str_value(self.raw.path_info)
   }

   pub fn args(&self) -> Option<&'a str> {
      c_str_value(self.raw.args)
   }

   pub fn log_id(&self) -> Option<&'a str> {
      c_str_value(self.raw.log_id)
   }

   pub fn useragent_ip(&self) -> Option<&'a str> {
      c_str_value(self.raw.useragent_ip)
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

   pub fn escape_html<T: Into<Vec<u8>>>(&self, s: T) -> Option<&'a str> {
      let c_str = CString::new(s).unwrap();

      let escaped = unsafe {
         ffi::ap_escape_html2(
            self.raw.pool,
            c_str.as_ptr(),
            0
         )
      };

      c_str_value(escaped)
   }

   pub fn escape_urlencoded<T: Into<Vec<u8>>>(&self, s: T) -> Option<&'a str> {
      let c_str = CString::new(s).unwrap();

      let escaped = unsafe {
         ffi::ap_escape_urlencoded(self.raw.pool, c_str.as_ptr())
      };

      c_str_value(escaped)
   }

   pub fn unescape_urlencoded<T: Into<Vec<u8>>>(&self, query: T) -> Option<&'a str> {
      let c_str = ffi::dup_c_str(self.raw.pool, query);

      let res = unsafe {
         ffi::ap_unescape_urlencoded(c_str)
      };

      if res != 0 {
         return None
      };

      c_str_value(c_str)
   }

   pub fn server_name(&self) -> Option<&'a str> {
      c_str_value(
         unsafe { ffi::ap_get_server_name(self.raw) }
      )
   }

   pub fn server_port(&self) -> u16 {
      unsafe { ffi::ap_get_server_port(self.raw) }
   }

   pub fn document_root(&self) -> Option<&'a str> {
      c_str_value(
         unsafe { ffi::ap_document_root(self.raw) }
      )
   }

   pub fn auth_name(&self) -> Option<&'a str> {
      c_str_value(
         unsafe { ffi::ap_auth_name(self.raw) }
      )
   }

   pub fn basic_auth_pw(&self) -> Option<&'a str> {
      let mut pw: *const c_char = ::std::ptr::null_mut();

      unsafe {
         ffi::ap_get_basic_auth_pw(self.raw, &mut pw);
      }

      c_str_value(pw)

   }

   pub fn context_document_root(&self) -> Option<&'a str> {
      c_str_value(
         unsafe { ffi::ap_context_document_root(self.raw) }
      )
   }

   pub fn context_prefix(&self) -> Option<&'a str> {
      c_str_value(
         unsafe { ffi::ap_context_prefix(self.raw) }
      )
   }

   pub fn http_scheme(&self) -> Option<&'a str> {
      c_str_value(
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

   pub fn cookie<T: Into<Vec<u8>>>(&self, name: T) -> Option<&'a str> {
      let c_str_name = ffi::dup_c_str(self.raw.pool, name);
      let mut val: *const c_char = ::std::ptr::null_mut();

      unsafe {
         ffi::ap_cookie_read(self.raw, c_str_name, &mut val, 0);
      }

      c_str_value(val)
   }

   pub fn set_cookie<T: Into<Vec<u8>>>(&self, name: T, val: T, maxage: i32) {
      let c_str_name = ffi::dup_c_str(self.raw.pool, name);
      let c_str_val = ffi::dup_c_str(self.raw.pool, val);

      let args = if self.http_scheme().unwrap() == "https" {
         "Secure;HttpOnly"
      } else {
         "HttpOnly"
      };
      let c_str_args = ffi::dup_c_str(self.raw.pool, args);

      let null: *const ffi::apr_table_t = ::std::ptr::null();

      unsafe {
         ffi::ap_cookie_write(self.raw, c_str_name, c_str_val, c_str_args, maxage,
            self.raw.err_headers_out, null);
      }
   }

   pub fn base64_encode<T: Into<Vec<u8>>>(&self, plain: T) -> Option<&'a str> {
      let c_str_plain: CString = match CString::new(plain) {
         Ok(val) => val,
         Err(_) => return None
      };

      let plain_len: c_int = c_str_plain.to_bytes().len() as c_int;

      let mut encoded_len: c_int = unsafe {
         ffi::apr_base64_encode_len(plain_len)
      };

      if encoded_len == 0 {
         return None
      };

      let encoded: *mut c_char = unsafe {
         ffi::apr_palloc(self.raw.pool, encoded_len as ffi::apr_size_t) as *mut c_char
      };

      encoded_len = unsafe {
         ffi::apr_base64_encode(encoded, c_str_plain.as_ptr(), plain_len)
      };

      if encoded_len == 0 {
         return None
      };

      c_str_value(encoded)
   }

   pub fn base64_decode<T: Into<Vec<u8>>>(&self, encoded: T) -> Option<&'a str> {
      let c_str_encoded: CString = match CString::new(encoded) {
         Ok(val) => val,
         Err(_) => return None
      };

      let mut plain_len: c_int = unsafe {
         ffi::apr_base64_decode_len(c_str_encoded.as_ptr())
      };

      if plain_len == 0 {
         return None
      };

      let plain: *mut c_char = unsafe {
         ffi::apr_palloc(self.raw.pool, plain_len as ffi::apr_size_t) as *mut c_char
      };

      plain_len = unsafe {
         ffi::apr_base64_decode(plain, c_str_encoded.as_ptr())
      };

      if plain_len == 0 {
         return None
      };

      c_str_value(plain)
   }
}

pub type Conn<'a> = Wrapper<'a, ffi::conn_rec>;


impl<'a> Conn<'a> {
   pub fn client_ip(&self) -> Option<&'a str> {
      c_str_value(self.raw.client_ip)
   }

   pub fn remote_host(&self) -> Option<&'a str> {
      c_str_value(self.raw.remote_host)
   }

   pub fn remote_logname(&self) -> Option<&'a str> {
      c_str_value(self.raw.remote_logname)
   }

   pub fn local_ip(&self) -> Option<&'a str> {
      c_str_value(self.raw.local_ip)
   }

   pub fn local_host(&self) -> Option<&'a str> {
      c_str_value(self.raw.local_host)
   }

   pub fn log_id(&self) -> Option<&'a str> {
      c_str_value(self.raw.log_id)
   }
}


pub fn server_banner<'a>() -> Option<&'a str> {
   c_str_value(
      unsafe { ffi::ap_get_server_banner() }
   )
}

pub fn server_description<'a>() -> Option<&'a str> {
   c_str_value(
      unsafe { ffi::ap_get_server_description() }
   )
}

pub fn server_built<'a>() -> Option<&'a str> {
   c_str_value(
      unsafe { ffi::ap_get_server_built() }
   )
}

pub fn show_mpm<'a>() -> Option<&'a str> {
   c_str_value(
      unsafe { ffi::ap_show_mpm() }
   )
}
