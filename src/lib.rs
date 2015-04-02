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

pub mod wrapper;

use libc::{c_void, c_char, c_int};

use std::ptr;

use apr::raw::{apr_pool_t, APR_HOOK_MIDDLE};

use httpd::raw::{request_rec};
use ap_exports::raw::{ap_hook_handler};
use http_config::raw::{command_rec};


macro_rules! module_def {
   ($module:ident, $handler:ident, $c_name:expr) => {
      const C_NAME: &'static [u8] = $c_name;
      const C_NAME_PTR: *const &'static [u8] = &C_NAME;
      const C_NAME_CHAR_PTR: *const c_char = C_NAME_PTR as *const c_char;

      #[no_mangle]
      pub static mut $module: http_config::raw::module = http_config::raw::module {
         version: ap_mmn::raw::MODULE_MAGIC_NUMBER_MAJOR,
         minor_version: ap_mmn::raw::MODULE_MAGIC_NUMBER_MINOR,
         module_index: -1,
         name: C_NAME_CHAR_PTR,
         dynamic_load_handle: 0 as *mut c_void,
         next: 0 as *mut http_config::raw::module,
         magic: ap_mmn::raw::MODULE_MAGIC_COOKIE,
         rewrite_args: None,
         create_dir_config: None,
         merge_dir_config: None,
         create_server_config: None,
         merge_server_config: None,
         cmds: 0 as *const command_rec,
         register_hooks: Some(c_module_hooks),
      };

      extern "C" fn c_module_hooks(_: *mut apr_pool_t) {
         unsafe {
            ap_hook_handler(
               Some(c_module_handler), ptr::null(), ptr::null(), APR_HOOK_MIDDLE
            );
         }
      }

      #[no_mangle]
      pub extern "C" fn c_module_handler(r: *mut request_rec) -> c_int {
         match httpd::Request::from_raw_ptr(r) {
            None => httpd::Status::DECLINED.into(),
            Some(request) => $handler(&request).into()
         }
      }
   }
}

module_def!(aprust_module, aprust_handler, b"mod_aprust\0");

fn dump_str<T: Into<Vec<u8>>>(req: &httpd::Request, name: T, optional: Option<&str>) {
   req.write("<p>");
   req.write(name);
   req.write(": ");
   req.write(
      match optional {
         None => "NULL",
         Some(slice) => slice
      }
   );
   req.write("</p>");
}

fn aprust_handler(req: &httpd::Request) -> httpd::Status {
   let mut headers_out = req.headers_out().unwrap();
   headers_out.set("Test-Key", "Hello");

   let headers_in = req.headers_in().unwrap();
   req.write("<html><head><meta charset=\"utf-8\"></head><body>");

   dump_str(req, "Cookie", headers_in.get("Cookie"));

   dump_str(req, "the_request", req.the_request());
   dump_str(req, "protocol", req.protocol());
   dump_str(req, "hostname", req.hostname());
   dump_str(req, "status_line", req.status_line());
   dump_str(req, "method", req.method());
   dump_str(req, "range", req.range());
   dump_str(req, "content_type", req.content_type());
   dump_str(req, "handler", req.handler());
   dump_str(req, "content_encoding", req.content_encoding());
   dump_str(req, "vlist_validator", req.vlist_validator());
   dump_str(req, "user", req.user());
   dump_str(req, "ap_auth_type", req.ap_auth_type());
   dump_str(req, "unparsed_uri", req.unparsed_uri());
   dump_str(req, "uri", req.uri());
   dump_str(req, "filename", req.filename());
   dump_str(req, "canonical_filename", req.canonical_filename());
   dump_str(req, "path_info", req.path_info());
   dump_str(req, "args", req.args());
   dump_str(req, "log_id", req.log_id());
   dump_str(req, "useragent_ip", req.useragent_ip());

   req.write("</body></html>");

   httpd::Status::OK
}
