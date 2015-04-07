pub mod raw {
   use libc::{c_char};

   use httpd::raw::{request_rec};
   use apr::raw::{apr_port_t};

   extern "C" {
      pub fn ap_get_server_name(r: *mut request_rec) -> *const c_char;
      pub fn ap_get_server_port(r: *mut request_rec) -> apr_port_t;
   }
}
