pub mod raw {
   use libc::{c_void, c_char, c_int};

   use httpd::raw::{request_rec};

   extern "C" {
      pub fn ap_rwrite(buf: *const c_void, nbyte: c_int, r: *const request_rec) -> c_int;
      pub fn ap_set_content_type(r: *const request_rec, ct: *const c_char) -> ();
      pub fn ap_get_basic_auth_pw(r: *const request_rec, pw: *mut *const c_char) -> c_int;
   }
}
