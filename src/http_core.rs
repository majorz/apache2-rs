pub mod raw {
   use libc::{c_char};

   use httpd::raw::{request_rec};

   extern "C" {
      pub fn ap_get_server_name(r: *mut request_rec) -> *const c_char;
   }
}
