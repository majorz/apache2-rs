extern crate libc;

pub mod apr;
pub mod ap_exports;

pub mod util_filter;
pub mod httpd;

pub mod ap_mmn;
pub mod http_config;

pub mod http_core;
pub mod http_protocol;

pub mod wrapper;


pub use httpd::{Request, Status, get_server_description, get_server_built};
pub use http_config::{show_mpm};
pub use apr::{apr_version_string};


#[macro_export]
macro_rules! apache2_module {
   ($module:ident, $handler:ident, $c_handler:ident, $c_name:expr) => {
      const C_NAME: &'static [u8] = $c_name;
      const C_NAME_PTR: *const &'static [u8] = &C_NAME;
      const C_NAME_CHAR_PTR: *const libc::c_char = C_NAME_PTR as *const libc::c_char;

      #[no_mangle]
      pub static mut $module: $crate::http_config::raw::module = $crate::http_config::raw::module {
         version: $crate::ap_mmn::raw::MODULE_MAGIC_NUMBER_MAJOR,
         minor_version: $crate::ap_mmn::raw::MODULE_MAGIC_NUMBER_MINOR,
         module_index: -1,
         name: C_NAME_CHAR_PTR,
         dynamic_load_handle: 0 as *mut libc::c_void,
         next: 0 as *mut $crate::http_config::raw::module,
         magic: $crate::ap_mmn::raw::MODULE_MAGIC_COOKIE,
         rewrite_args: None,
         create_dir_config: None,
         merge_dir_config: None,
         create_server_config: None,
         merge_server_config: None,
         cmds: 0 as *const $crate::http_config::raw::command_rec,
         register_hooks: Some(c_module_hooks),
      };

      extern "C" fn c_module_hooks(_: *mut $crate::apr::raw::apr_pool_t) {
         unsafe {
            $crate::ap_exports::raw::ap_hook_handler(
               Some($c_handler),
               std::ptr::null(),
               std::ptr::null(),
               $crate::apr::raw::APR_HOOK_MIDDLE
            );
         }
      }

      #[no_mangle]
      pub extern "C" fn $c_handler(r: *mut $crate::httpd::raw::request_rec) -> libc::c_int {
         match $crate::httpd::Request::from_raw_ptr(r) {
            None => $crate::httpd::Status::DECLINED.into(),
            Some(request) => $handler(&request).into()
         }
      }
   }
}
