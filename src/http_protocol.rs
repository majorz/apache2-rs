pub mod raw {
   use libc::{c_void, c_int};

   use httpd::raw::{request_rec};

   extern "C" {
      pub fn ap_rwrite(buf: *const c_void, nbyte: c_int, r: *mut request_rec) -> c_int;
   }
}
