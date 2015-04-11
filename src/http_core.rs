pub mod raw {
   use libc::{c_char};

   use httpd::raw::{request_rec};
   use apr::raw::{apr_port_t};

   extern "C" {
      pub fn ap_document_root(r: *const request_rec) -> *const c_char;
      pub fn ap_get_server_name(r: *const request_rec) -> *const c_char;
      pub fn ap_get_server_port(r: *const request_rec) -> apr_port_t;
      pub fn ap_auth_name(r: *const request_rec) -> *const c_char;
   }
}
