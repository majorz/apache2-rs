#![feature(libc)]
#![feature(convert)]

extern crate libc;

pub mod apr;
pub mod ap_exports;

pub mod util_filter;
pub mod httpd;

pub mod ap_mmn;
pub mod http_config;

pub mod http_protocol;

use libc::{c_void, c_char, c_int};

use std::ptr;
use std::str;
use std::ffi::{CString, CStr};

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

fn dump_str<T: Into<Vec<u8>>>(r: *mut request_rec, name: T, p: *const c_char) {
   rwrite("<p>", r);
   rwrite(name, r);
   rwrite(": ", r);

   if p.is_null() {
      rwrite("NULL", r);
   } else {
      let data =  unsafe { CStr::from_ptr(p) };
      let bytes = data.to_bytes();

      let slice = str::from_utf8(bytes).unwrap();

      let html = format!("{:?}", slice);

      rwrite(html, r);
   }

   rwrite("</p>", r);
}

#[no_mangle]
pub extern "C" fn aprust_handler(r: *mut request_rec) -> c_int {
   let w = httpd::Request::from_raw(r).unwrap();

   let req: &request_rec = unsafe { &*r };

   rwrite("<html><head><meta charset=\"utf-8\"></head><body>", r);

   rwrite(w.the_request().unwrap(), r);

   dump_str(r, "the_request", req.the_request);
   dump_str(r, "protocol", req.protocol);
   dump_str(r, "hostname", req.hostname);
   dump_str(r, "status_line", req.status_line);
   dump_str(r, "method", req.method);
   dump_str(r, "range", req.range);
   dump_str(r, "content_type", req.content_type);
   dump_str(r, "handler", req.handler);
   dump_str(r, "content_encoding", req.content_encoding);
   dump_str(r, "vlist_validator", req.vlist_validator);
   dump_str(r, "user", req.user);
   dump_str(r, "ap_auth_type", req.ap_auth_type);
   dump_str(r, "unparsed_uri", req.unparsed_uri);
   dump_str(r, "uri", req.uri);
   dump_str(r, "filename", req.filename);
   dump_str(r, "canonical_filename", req.canonical_filename);
   dump_str(r, "path_info", req.path_info);
   dump_str(r, "args", req.args);
   dump_str(r, "log_id", req.log_id);
   dump_str(r, "useragent_ip", req.useragent_ip);

   rwrite("</body></html>", r);

   OK
}
