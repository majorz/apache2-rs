#![allow(non_camel_case_types)]

use libc::{c_void, c_char, c_uchar, c_short, c_ushort, c_int, c_uint, c_long, c_ulong};

// APACHE PORTABLE RUNTIME

// run this hook first, before ANYTHING
pub const APR_HOOK_REALLY_FIRST:  c_int = -10;
// run this hook first
pub const APR_HOOK_FIRST:         c_int = 0;
// run this hook somewhere
pub const APR_HOOK_MIDDLE:        c_int = 10;
// run this hook after every other hook which is defined
pub const APR_HOOK_LAST:          c_int = 20;
// run this hook last, after EVERYTHING
pub const APR_HOOK_REALLY_LAST:   c_int = 30;

pub type apr_byte_t = c_uchar;
pub type apr_int16_t = c_short;
pub type apr_uint16_t = c_ushort;
pub type apr_int32_t = c_int;
pub type apr_uint32_t = c_uint;
pub type apr_int64_t = c_long;
pub type apr_uint64_t = c_ulong;
pub type apr_size_t = c_ulong;
pub type apr_ssize_t = c_long;
pub type apr_off_t = c_long;
pub type apr_socklen_t = c_uint;
pub type apr_ino_t = c_ulong;
pub type apr_uintptr_t = apr_uint64_t;
pub type apr_status_t = c_int;
pub type apr_signum_t = c_int;
pub type apr_time_t = apr_int64_t;
pub type apr_port_t = apr_uint16_t;

#[repr(C)]
pub struct apr_array_header_t {
   pub pool: *mut apr_pool_t,
   pub elt_size: c_int,
   pub nelts: c_int,
   pub nalloc: c_int,
   pub elts: *mut c_char,
}

#[repr(C)]
pub struct apr_table_entry_t {
   pub key: *mut c_char,
   pub val: *mut c_char,
   pub key_checksum: apr_uint32_t,
}

#[repr(C)]
pub struct apr_bucket_alloc_t;

#[repr(C)]
pub struct apr_bucket_brigade;

#[repr(C)]
pub struct apr_finfo_t;

#[repr(C)]
pub struct apr_pool_t;

#[repr(C)]
pub struct apr_sockaddr_t;

#[repr(C)]
pub struct apr_table_t;

#[repr(C)]
pub struct apr_thread_mutex_t;

#[repr(C)]
pub struct apr_thread_t;

#[repr(C)]
pub struct apr_uri_t;

extern "C" {
   pub fn apr_version_string() -> *const c_char;
   pub fn apu_version_string() -> *const c_char;

   pub fn apr_table_get(t: *const apr_table_t, key: *const c_char) -> *const c_char;
   pub fn apr_table_set(t: *const apr_table_t, key: *const c_char, val: *const c_char) -> ();
   pub fn apr_table_elts(t: *const apr_table_t) -> *const apr_array_header_t;

   pub fn apr_pstrmemdup(p: *mut apr_pool_t, s: *const c_char, n: apr_size_t) -> *mut c_char;
   pub fn apr_palloc(p: *mut apr_pool_t, size: apr_size_t) -> *mut c_void;

   pub fn apr_base64_encode_len(len: c_int) -> c_int;
   pub fn apr_base64_encode(coded_dst: *mut c_char, plain_src: *const c_char, len_plain_src: c_int) -> c_int;
   pub fn apr_base64_decode_len(coded_src: *const c_char) -> c_int;
   pub fn apr_base64_decode(plain_dst: *mut c_char, coded_src: *const c_char) -> c_int;
}

pub fn dup_c_str<T: Into<Vec<u8>>>(pool: *mut apr_pool_t, data: T) -> *mut c_char {
   let bytes = data.into();

   unsafe {
      apr_pstrmemdup(
         pool,
         bytes.as_ptr() as *const c_char,
         bytes.len() as apr_size_t
      )
   }
}

// APACHE HTTPD

pub const MODULE_MAGIC_COOKIE: c_ulong = 0x41503234u64; /* "AP24" */

pub const MODULE_MAGIC_NUMBER_MAJOR: c_int = 20120211;
pub const MODULE_MAGIC_NUMBER_MINOR: c_int = 36;

pub const OK:        c_int = 0;
pub const DECLINED:  c_int = -1;
pub const DONE:      c_int = -2;
pub const SUSPENDED: c_int = -3;

pub const  PROXYREQ_NONE:     c_int = 0;
pub const  PROXYREQ_PROXY:    c_int = 1;
pub const  PROXYREQ_REVERSE:  c_int = 2;
pub const  PROXYREQ_RESPONSE: c_int = 3;

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
pub struct module {
   pub version: c_int,
   pub minor_version: c_int,
   pub module_index: c_int,
   pub name: *const c_char,
   pub dynamic_load_handle: *mut c_void,
   pub next: *mut module,
   pub magic: c_ulong,
   pub rewrite_args: Option<rewrite_args_fn>,
   pub create_dir_config: Option<create_dir_config_fn>,
   pub merge_dir_config: Option<merge_config_fn>,
   pub create_server_config: Option<create_server_config_fn>,
   pub merge_server_config: Option<merge_config_fn>,
   pub cmds: *const command_rec,
   pub register_hooks: ::std::option::Option<register_hooks_fn>
}

#[repr(C)]
pub struct ap_filter_t;

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

#[repr(C)]
pub struct command_rec;

pub type rewrite_args_fn = extern "C" fn(
   process: *mut process_rec
);

pub type create_dir_config_fn = extern "C" fn(
   p: *mut apr_pool_t, dir: *mut c_char
) -> *mut c_void;

pub type merge_config_fn = extern "C" fn(
   p: *mut apr_pool_t, base_conf: *mut c_void, new_conf: *mut c_void
) -> *mut c_void;

pub type create_server_config_fn = extern "C" fn(
   p: *mut apr_pool_t, s: *mut server_rec
) -> *mut c_void;

pub type register_hooks_fn = extern "C" fn(
   p: *mut apr_pool_t
);

pub type hook_handler_fn = extern "C" fn(
   r: *const request_rec
) -> c_int;


extern "C" {
   pub fn ap_hook_handler(hook_handler: Option<hook_handler_fn>,
      pre: *const *const c_char, succ: *const *const c_char, order: c_int);

   pub fn ap_get_server_banner() -> *const c_char;
   pub fn ap_get_server_description() -> *const c_char;
   pub fn ap_get_server_built() -> *const c_char;

   pub fn ap_show_mpm() -> *const c_char;

   pub fn ap_escape_html2(p: *mut apr_pool_t, s: *const c_char, toasc: c_int) -> *mut c_char;

   pub fn ap_rwrite(buf: *const c_void, nbyte: c_int, r: *const request_rec) -> c_int;
   pub fn ap_set_content_type(r: *const request_rec, ct: *const c_char) -> ();
   pub fn ap_get_basic_auth_pw(r: *const request_rec, pw: *mut *const c_char) -> c_int;

   pub fn ap_context_document_root(r: *const request_rec) -> *const c_char;
   pub fn ap_context_prefix(r: *const request_rec) -> *const c_char;

   pub fn ap_run_http_scheme(r: *const request_rec) -> *const c_char;
   pub fn ap_run_default_port(r: *const request_rec) -> apr_port_t;

   pub fn ap_is_initial_req(r: *const request_rec) -> c_int;

   pub fn ap_some_auth_required(r: *const request_rec) -> c_int;

   pub fn ap_cookie_read(r: *const request_rec, name: *const c_char, val: *mut *const c_char,
      remove: c_int) -> apr_status_t;
   pub fn ap_cookie_write(r: *const request_rec, name: *const c_char, val: *const c_char,
      attrs: *const c_char, maxage: c_int, ...) -> apr_status_t;

   pub fn ap_escape_urlencoded(p: *mut apr_pool_t, s: *const c_char) -> *mut c_char;
   pub fn ap_unescape_urlencoded(query: *mut c_char) -> c_int;

   pub fn ap_document_root(r: *const request_rec) -> *const c_char;
   pub fn ap_get_server_name(r: *const request_rec) -> *const c_char;
   pub fn ap_get_server_port(r: *const request_rec) -> apr_port_t;
   pub fn ap_auth_name(r: *const request_rec) -> *const c_char;
}
