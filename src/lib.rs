extern crate libc;

pub mod ffi;
pub mod apr;
pub mod httpd;
pub mod wrapper;
pub mod cookie;

pub use libc::{c_void, c_char, c_int};

pub use httpd::{Request, Status, ProxyReq, server_banner, server_description, server_built, show_mpm};
pub use apr::{apr_version_string, apu_version_string, HookOrder, time_now};
pub use cookie::Cookie;


#[macro_export]
macro_rules! AP_DECLARE_MODULE {
   (
      $module:ident,
      $name:expr,
      $create_dir_config:expr,
      $merge_dir_config:expr,
      $create_server_config:expr,
      $merge_server_config:expr,
      $cmds:expr,
      $register_hooks:expr
   ) => {
      #[no_mangle]
      pub static mut $module: $crate::ffi::module = $crate::ffi::module {
         version: $crate::ffi::MODULE_MAGIC_NUMBER_MAJOR,
         minor_version: $crate::ffi::MODULE_MAGIC_NUMBER_MINOR,
         module_index: -1,
         name: $name as *const u8 as *const $crate::c_char,
         dynamic_load_handle: 0 as *mut $crate::c_void,
         next: 0 as *mut $crate::ffi::module,
         magic: $crate::ffi::MODULE_MAGIC_COOKIE,
         rewrite_args: None,
         create_dir_config: $create_dir_config,
         merge_dir_config: $merge_dir_config,
         create_server_config: $create_server_config,
         merge_server_config: $merge_server_config,
         cmds: $cmds as *const $crate::ffi::command_rec,
         register_hooks: $register_hooks
      };
   }
}


#[macro_export]
macro_rules! apache2_module {
   ($handler:ident) => {
      apache2_module!($handler, c_hello_handler, hello_module, b"mod_hello\0");
   };

   ($handler:ident, $c_handler:ident, $module:ident, $c_name:expr) => {
      apache2_module!($handler, $c_handler, $module, $c_name, ap_hook_handler, $crate::HookOrder::MIDDLE);
   };

   ($handler:ident, $c_handler:ident, $module:ident, $c_name:expr, $hook:ident, $order:expr) => {
      AP_DECLARE_MODULE!(
         $module,
         $c_name,
         None,
         None,
         None,
         None,
         0,
         Some(c_module_hooks)
      );

      extern "C" fn c_module_hooks(_: *mut $crate::ffi::apr_pool_t) {
         unsafe {
            $crate::ffi::$hook(
               Some($c_handler),
               std::ptr::null(),
               std::ptr::null(),
               $order.into()
            );
         }
      }

      #[no_mangle]
      pub extern "C" fn $c_handler(r: *mut $crate::ffi::request_rec) -> $crate::c_int {
         match $crate::httpd::Request::from_raw_ptr(r) {
            Err(_) => $crate::httpd::Status::DECLINED.into(),
            Ok(mut request) => $handler(&mut request).into()
         }
      }
   }
}
