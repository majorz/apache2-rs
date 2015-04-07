pub mod raw {
   use libc::{c_char, c_int, c_uint, c_long, c_void};

   use apr::raw::{apr_pool_t, apr_time_t, apr_array_header_t, apr_off_t, apr_thread_mutex_t,
      apr_int64_t, apr_table_t, apr_bucket_brigade, apr_uri_t, apr_finfo_t, apr_sockaddr_t,
      apr_thread_t, apr_bucket_alloc_t};

   use util_filter::raw::{ap_filter_t};

   pub const OK:        c_int = 0;
   pub const DECLINED:  c_int = -1;
   pub const DONE:      c_int = -2;
   pub const SUSPENDED: c_int = -3;

   #[repr(C)]
   pub struct request_rec {
      pub pool: *mut apr_pool_t,
      pub connection: *mut conn_rec,
      pub server: *mut server_rec,
      pub next: *mut request_rec,
      pub prev: *mut request_rec,
      pub main: *mut request_rec,
      pub the_request: *mut c_char,
      pub assbackwards: c_int,
      pub proxyreq: c_int,
      pub header_only: c_int,
      pub proto_num: c_int,
      pub protocol: *mut c_char,
      pub hostname: *const c_char,
      pub request_time: apr_time_t,
      pub status_line: *const c_char,
      pub status: c_int,
      pub method_number: c_int,
      pub method: *const c_char,
      pub allowed: apr_int64_t,
      pub allowed_xmethods: *mut apr_array_header_t,
      pub allowed_methods: *mut ap_method_list_t,
      pub sent_bodyct: apr_off_t,
      pub bytes_sent: apr_off_t,
      pub mtime: apr_time_t,
      pub range: *const c_char,
      pub clength: apr_off_t,
      pub chunked: c_int,
      pub read_body: c_int,
      pub read_chunked: c_int,
      pub expecting_100: c_uint,
      pub kept_body: *mut apr_bucket_brigade,
      pub body_table: *mut apr_table_t,
      pub remaining: apr_off_t,
      pub read_length: apr_off_t,
      pub headers_in: *mut apr_table_t,
      pub headers_out: *mut apr_table_t,
      pub err_headers_out: *mut apr_table_t,
      pub subprocess_env: *mut apr_table_t,
      pub notes: *mut apr_table_t,
      pub content_type: *const c_char,
      pub handler: *const c_char,
      pub content_encoding: *const c_char,
      pub content_languages: *mut apr_array_header_t,
      pub vlist_validator: *mut c_char,
      pub user: *mut c_char,
      pub ap_auth_type: *mut c_char,
      pub unparsed_uri: *mut c_char,
      pub uri: *mut c_char,
      pub filename: *mut c_char,
      pub canonical_filename: *mut c_char,
      pub path_info: *mut c_char,
      pub args: *mut c_char,
      pub used_path_info: c_int,
      pub eos_sent: c_int,
      pub per_dir_config: *mut ap_conf_vector_t,
      pub request_config: *mut ap_conf_vector_t,
      pub log: *const ap_logconf,
      pub log_id: *const c_char,
      pub htaccess: *const htaccess_result,
      pub output_filters: *mut ap_filter_t,
      pub input_filters: *mut ap_filter_t,
      pub proto_output_filters: *mut ap_filter_t,
      pub proto_input_filters: *mut ap_filter_t,
      pub no_cache: c_int,
      pub no_local_copy: c_int,
      pub invoke_mtx: *mut apr_thread_mutex_t,
      pub parsed_uri: apr_uri_t,
      pub finfo: apr_finfo_t,
      pub useragent_addr: *mut apr_sockaddr_t,
      pub useragent_ip: *mut c_char,
      pub trailers_in: *mut apr_table_t,
      pub trailers_out: *mut apr_table_t,
   }

   #[repr(C)]
   pub struct conn_rec {
      pub pool: *mut apr_pool_t,
      pub base_server: *mut server_rec,
      pub vhost_lookup_data: *mut c_void,
      pub local_addr: *mut apr_sockaddr_t,
      pub client_addr: *mut apr_sockaddr_t,
      pub client_ip: *mut c_char,
      pub remote_host: *mut c_char,
      pub remote_logname: *mut c_char,
      pub local_ip: *mut c_char,
      pub local_host: *mut c_char,
      pub id: c_long,
      pub conn_config: *mut ap_conf_vector_t,
      pub notes: *mut apr_table_t,
      pub input_filters: *mut ap_filter_t,
      pub output_filters: *mut ap_filter_t,
      pub sbh: *mut c_void,
      pub bucket_alloc: *mut apr_bucket_alloc_t,
      pub cs: *mut conn_state_t,
      pub data_in_input_filters: c_int,
      pub data_in_output_filters: c_int,
      pub _bindgen_bitfield_1_: c_uint,
      pub _bindgen_bitfield_2_: c_int,
      pub aborted: c_uint,
      pub keepalive: ap_conn_keepalive_e,
      pub keepalives: c_int,
      pub log: *const ap_logconf,
      pub log_id: *const c_char,
      pub current_thread: *mut apr_thread_t,
   }

   #[repr(C)]
   pub struct ap_logconf {
       pub module_levels: *mut c_char,
       pub level: c_int,
   }

   #[repr(C)]
   pub struct ap_conn_keepalive_e;

   #[repr(C)]
   pub struct ap_conf_vector_t;

   #[repr(C)]
   pub struct ap_method_list_t;

   #[repr(C)]
   pub struct conn_state_t;

   #[repr(C)]
   pub struct htaccess_result;

   #[repr(C)]
   pub struct process_rec;

   #[repr(C)]
   pub struct server_rec;

   extern "C" {
      pub fn ap_get_server_banner() -> *const c_char;

      pub fn ap_get_server_description() -> *const c_char;

      pub fn ap_get_server_built() -> *const c_char;

      pub fn ap_escape_html2(p: *mut apr_pool_t, s: *const c_char, toasc: c_int) -> *mut c_char;
   }
}


use libc::{c_void, c_int};

use std::ffi::CString;

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
         Status::OK => raw::OK,
         Status::DECLINED => raw::DECLINED,
         Status::DONE => raw::DONE,
         Status::SUSPENDED => raw::SUSPENDED,
      }
   }
}

pub type Request<'a> = Wrapper<'a, raw::request_rec>;


impl<'a> Request<'a> {
   pub fn connection(&self) -> Option<Conn> {
      wrap_ptr(self.raw.connection)
   }

   pub fn the_request(&self) -> Option<&'a str> {
      c_str_value(self.raw.the_request)
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
      let c_str = ::apr::raw::dup_c_str(self.raw.pool, ct);

      unsafe {
         ::http_protocol::raw::ap_set_content_type(
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

   pub fn ap_auth_type(&self) -> Option<&'a str> {
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
      let cstr = CString::new(data).unwrap();

      unsafe {
         ::http_protocol::raw::ap_rwrite(
            cstr.as_ptr() as *mut c_void,
            cstr.to_bytes().len() as i32,
            self.raw
         );
      }
   }

   pub fn escape_html<T: Into<Vec<u8>>>(&self, s: T) -> Option<&'a str> {
      let cstr = CString::new(s).unwrap();

      let escaped = unsafe {
         raw::ap_escape_html2(
            self.raw.pool,
            cstr.as_ptr(),
            0
         )
      };

      c_str_value(escaped)
   }

   pub fn server_name(&self) -> Option<&'a str> {
      c_str_value(
         unsafe { ::http_core::raw::ap_get_server_name(self.raw) }
      )
   }

   pub fn server_port(&self) -> u16 {
      unsafe { ::http_core::raw::ap_get_server_port(self.raw) }
   }
}

pub type Conn<'a> = Wrapper<'a, raw::conn_rec>;


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
      unsafe { raw::ap_get_server_banner() }
   )
}

pub fn server_description<'a>() -> Option<&'a str> {
   c_str_value(
      unsafe { raw::ap_get_server_description() }
   )
}

pub fn server_built<'a>() -> Option<&'a str> {
   c_str_value(
      unsafe { raw::ap_get_server_built() }
   )
}
