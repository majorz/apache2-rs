extern crate libc;

pub mod ffi;
pub mod apr;
pub mod httpd;
pub mod wrapper;
pub mod cookie;

pub use libc::{c_void, c_char, c_int};

pub use httpd::{Request, Status, ProxyReq, server_banner, server_description, server_built,
   show_mpm};
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
         cmds: unsafe { $cmds as *const $crate::ffi::command_rec },
         register_hooks: $register_hooks
      };
   }
}


#[macro_export]
macro_rules! apache2_module {
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
            Ok(mut request) => match $handler(&mut request) {
               Ok(status) => status,
               Err(_) => $crate::httpd::Status::HTTP_INTERNAL_SERVER_ERROR
            }.into()
         }
      }
   }
}


#[macro_export]
macro_rules! DECLARE_COMMAND_REC {
   (
      $name:expr,
      $func:expr,
      $cmd_data:expr,
      $req_override:expr,
      $args_how:expr,
      $errmsg:expr
   ) => {
      ffi::command_rec {
         name: $name as *const u8 as *const c_char,
         func: ffi::cmd_func {
            _bindgen_data_: [$func as u64]
         },
         cmd_data: $cmd_data as *mut c_void,
         req_override: $req_override,
         args_how: $args_how,
         errmsg: $errmsg as *const u8 as *const c_char
      }
   }
}


#[macro_export]
macro_rules! NULL_COMMAND_REC {
   () => {
      DECLARE_COMMAND_REC!(0, 0, 0, 0, 0, 0)
   }
}

#[macro_export]
macro_rules! AP_INIT_NO_ARGS {
   ($name:expr, $func:expr, $cmd_data:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, $cmd_data, $req_override, ffi::NO_ARGS, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_RAW_ARGS {
   ($name:expr, $func:expr, $cmd_data:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, $cmd_data, $req_override, ffi::RAW_ARGS, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE_ARGV {
   ($name:expr, $func:expr, $cmd_data:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, $cmd_data, $req_override, ffi::TAKE_ARGV, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE1 {
   ($name:expr, $func:expr, $cmd_data:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, $cmd_data, $req_override, ffi::TAKE1, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_ITERATE {
   ($name:expr, $func:expr, $cmd_data:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, $cmd_data, $req_override, ffi::ITERATE, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE2 {
   ($name:expr, $func:expr, $cmd_data:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, $cmd_data, $req_override, ffi::TAKE2, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE12 {
   ($name:expr, $func:expr, $cmd_data:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, $cmd_data, $req_override, ffi::TAKE12, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_ITERATE2 {
   ($name:expr, $func:expr, $cmd_data:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, $cmd_data, $req_override, ffi::ITERATE2, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE13 {
   ($name:expr, $func:expr, $cmd_data:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, $cmd_data, $req_override, ffi::TAKE13, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE23 {
   ($name:expr, $func:expr, $cmd_data:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, $cmd_data, $req_override, ffi::TAKE23, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE123 {
   ($name:expr, $func:expr, $cmd_data:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, $cmd_data, $req_override, ffi::TAKE123, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE3 {
   ($name:expr, $func:expr, $cmd_data:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, $cmd_data, $req_override, ffi::TAKE3, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_FLAG {
   ($name:expr, $func:expr, $cmd_data:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, $cmd_data, $req_override, ffi::FLAG, $errmsg)
   }
}

#[macro_export]
macro_rules! apache2_commands {
   ($cmds_name:ident, $cmd_count:expr, $( $cmd:expr ),*) => {
      #[no_mangle]
      pub static mut $cmds_name: [ffi::command_rec; $cmd_count] = [
         $(
            $cmd,
         )*
         NULL_COMMAND_REC!()
      ];
   };

   ($cmds_name:ident, $cmd1:expr) => {
      apache2_commands!($cmds_name, 2, $cmd1);
   };

   ($cmds_name:ident, $cmd1:expr, $cmd2:expr) => {
      apache2_commands!($cmds_name, 3, $cmd1, $cmd2);
   };

   ($cmds_name:ident, $cmd1:expr, $cmd2:expr, $cmd3:expr) => {
      apache2_commands!($cmds_name, 9, $cmd1, $cmd2, $cmd3);
   };

   ($cmds_name:ident, $cmd1:expr, $cmd2:expr, $cmd3:expr, $cmd4:expr) => {
      apache2_commands!($cmds_name, 9, $cmd1, $cmd2, $cmd3, $cmd4);
   };

   ($cmds_name:ident, $cmd1:expr, $cmd2:expr, $cmd3:expr, $cmd4:expr, $cmd5:expr) => {
      apache2_commands!($cmds_name, 9, $cmd1, $cmd2, $cmd3, $cmd4, $cmd5);
   };

   ($cmds_name:ident, $cmd1:expr, $cmd2:expr, $cmd3:expr, $cmd4:expr, $cmd5:expr, $cmd6:expr) => {
      apache2_commands!($cmds_name, 9, $cmd1, $cmd2, $cmd3, $cmd4, $cmd5, $cmd6);
   };

   ($cmds_name:ident, $cmd1:expr, $cmd2:expr, $cmd3:expr, $cmd4:expr, $cmd5:expr, $cmd6:expr, $cmd7:expr) => {
      apache2_commands!($cmds_name, 9, $cmd1, $cmd2, $cmd3, $cmd4, $cmd5, $cmd6, $cmd7);
   };

   ($cmds_name:ident, $cmd1:expr, $cmd2:expr, $cmd3:expr, $cmd4:expr, $cmd5:expr, $cmd6:expr, $cmd7:expr, $cmd8:expr) => {
      apache2_commands!($cmds_name, 9, $cmd1, $cmd2, $cmd3, $cmd4, $cmd5, $cmd6, $cmd7, $cmd8);
   };
}
