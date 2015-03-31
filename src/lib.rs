#![feature(libc)]
#![feature(convert)]

extern crate libc;

pub mod apr;
pub mod ap_exports;

pub mod httpd;

pub mod ap_mmn;
pub mod http_config;

pub mod http_protocol;

use libc::{c_void, c_char, c_int};

use std::ptr;
use std::ffi::CString;

use apr::raw::{apr_pool_t, APR_HOOK_MIDDLE};

use httpd::raw::{OK, request_rec};
use ap_exports::raw::{ap_hook_handler};
use http_config::raw::{command_rec};
use http_protocol::raw::{ap_rwrite};


macro_rules! module_def {
   ($module:ident, $handler:ident, $cname:expr) => {
      const CNAME: &'static [u8] = $cname;
      const CNAME_PTR: *const &'static [u8] = &CNAME;
      const CNAME_CHAR_PTR: *const c_char = CNAME_PTR as *const c_char;

      #[no_mangle]
      pub static mut $module: http_config::raw::module = http_config::raw::module {
         version: ap_mmn::raw::MODULE_MAGIC_NUMBER_MAJOR,
         minor_version: ap_mmn::raw::MODULE_MAGIC_NUMBER_MINOR,
         module_index: -1,
         name: CNAME_CHAR_PTR,
         dynamic_load_handle: 0 as *mut c_void,
         next: 0 as *mut http_config::raw::module,
         magic: ap_mmn::raw::MODULE_MAGIC_COOKIE,
         rewrite_args: None,
         create_dir_config: None,
         merge_dir_config: None,
         create_server_config: None,
         merge_server_config: None,
         cmds: 0 as *const command_rec,
         register_hooks: Some(module_hooks),
      };

      extern "C" fn module_hooks(_: *mut apr_pool_t) {
         unsafe {
            ap_hook_handler(
               Some($handler), ptr::null(), ptr::null(), APR_HOOK_MIDDLE
            );
         }
      }

   }
}

module_def!(aprust_module, aprust_handler, b"mod_aprust\0");

fn rwrite<T: Into<Vec<u8>>>(t: T, r: *mut request_rec) {
   let s = CString::new(t).unwrap();
   let len = s.to_bytes().len();

   unsafe {
      ap_rwrite(s.as_ptr() as *mut c_void, len as i32, r);
   }
}

#[no_mangle]
pub extern "C" fn aprust_handler(r: *mut request_rec) -> c_int {
   rwrite("<html><head><meta charset=\"utf-8\"></head><body>Здравейте!</body></html>", r);

   OK
}
