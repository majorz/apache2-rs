#![feature(libc)]
#![feature(convert)]

extern crate libc;

pub mod apr {
   use libc::{c_int};

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


   #[repr(C)]
   pub struct apr_pool_t;
}

pub mod httpd {
   use libc::{c_int};

   #[repr(C)]
   pub struct process_rec;

   #[repr(C)]
   pub struct request_rec;

   #[repr(C)]
   pub struct server_rec;

   pub const OK:        c_int = 0;
   pub const DECLINED:  c_int = -1;
   pub const DONE:      c_int = -2;
   pub const SUSPENDED: c_int = -3;
}

pub mod ap_exports {
   #![allow(non_camel_case_types)]

   use libc::{c_int, c_char};

   use httpd::{request_rec};

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

pub mod ap_mmn {
   use libc::{c_int, c_ulong};

   pub const MODULE_MAGIC_COOKIE: c_ulong = 0x41503234u64; /* "AP24" */

   pub const MODULE_MAGIC_NUMBER_MAJOR: c_int = 20120211;
   pub const MODULE_MAGIC_NUMBER_MINOR: c_int = 36;
}

pub mod http_config {
   #![allow(non_camel_case_types)]

   use libc::{c_void, c_char, c_int, c_ulong};

   use apr::{apr_pool_t};

   use httpd::{process_rec, server_rec};

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

   pub type module = module_struct;

   #[repr(C)]
   pub struct module_struct {
      pub version: c_int,

      pub minor_version: c_int,

      pub module_index: c_int,

      pub name: *const c_char,

      pub dynamic_load_handle: *mut c_void,

      pub next: *mut module_struct,

      pub magic: c_ulong,

      pub rewrite_args: Option<rewrite_args_fn>,

      pub create_dir_config: Option<create_dir_config_fn>,

      pub merge_dir_config: Option<merge_config_fn>,

      pub create_server_config: Option<create_server_config_fn>,

      pub merge_server_config: Option<merge_config_fn>,

      pub cmds: *const command_rec,

      pub register_hooks: ::std::option::Option<register_hooks_fn>
   }
}

pub mod http_protocol {
   use libc::{c_void, c_int};

   use httpd::{request_rec};

   extern "C" {
      pub fn ap_rwrite(buf: *const c_void, nbyte: c_int, r: *mut request_rec) -> c_int;
   }
}

use libc::{c_void, c_char, c_int};

use std::ptr;
use std::ffi::CString;

use apr::{apr_pool_t, APR_HOOK_MIDDLE};

use httpd::{OK, request_rec};
use ap_exports::{ap_hook_handler};
use http_config::{command_rec};
use http_protocol::{ap_rwrite};


macro_rules! module_def {
   ($module:ident, $handler:ident, $cname:expr) => {
      const CNAME: &'static [u8] = $cname;
      const CNAME_PTR: *const &'static [u8] = &CNAME;
      const CNAME_CHAR_PTR: *const c_char = CNAME_PTR as *const c_char;

      #[no_mangle]
      pub static mut $module: http_config::module = http_config::module {
         version: ap_mmn::MODULE_MAGIC_NUMBER_MAJOR,
         minor_version: ap_mmn::MODULE_MAGIC_NUMBER_MINOR,
         module_index: -1,
         name: CNAME_CHAR_PTR,
         dynamic_load_handle: 0 as *mut c_void,
         next: 0 as *mut http_config::module,
         magic: ap_mmn::MODULE_MAGIC_COOKIE,
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
