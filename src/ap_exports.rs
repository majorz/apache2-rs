pub mod raw {
   #![allow(non_camel_case_types)]

   use libc::{c_int, c_char};

   use httpd::raw::{request_rec};

   pub type hook_handler_fn = extern "C" fn(
      r: *mut request_rec
   ) -> c_int;

   extern "C" {
      pub fn ap_hook_handler(
         hook_handler: Option<hook_handler_fn>,
         pre: *const *const c_char,
         succ: *const *const c_char,
         order: c_int
      );
   }
}
