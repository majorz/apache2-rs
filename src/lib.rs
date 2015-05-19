#![feature(plugin)]
#![plugin(interpolate_idents)]

extern crate libc;

pub mod ffi;
pub mod apr;
pub mod httpd;
pub mod wrapper;
pub mod cookie;

pub use libc::{c_void, c_char, c_int};

pub use httpd::{Request, Status, ProxyReq, CmdParms, Server, server_banner, server_description,
   server_built, show_mpm, ConfVector};

pub use apr::{apr_version_string, apu_version_string, HookOrder, Pool, time_now};

pub use cookie::Cookie;

pub use wrapper::{CType, from_char_ptr};

pub use ffi::{OR_NONE, OR_LIMIT, OR_OPTIONS, OR_FILEINFO, OR_AUTHCFG, OR_INDEXES, OR_UNSET,
   ACCESS_CONF, RSRC_CONF, EXEC_ON_READ, NONFATAL_OVERRIDE, NONFATAL_UNKNOWN, NONFATAL_ALL, OR_ALL,
   RAW_ARGS, TAKE1, TAKE2, ITERATE, ITERATE2, FLAG, NO_ARGS, TAKE12, TAKE3, TAKE23, TAKE123,
   TAKE13, TAKE_ARGV};


#[macro_export]
macro_rules! apache2_module {
   ($name:ident, $mod_name:expr) => {
      interpolate_idents! {
         apache2_module!(
            $name,
            $mod_name,
            handlers {
               [$name _handler], handler, $crate::HookOrder::MIDDLE
            }
         );
      }
   };

   ($name:ident, $mod_name:expr, handlers { $handler:ident, $hook:ident, $order:expr }) => {
      apache2_module!(
         $name,
         $mod_name,
         handlers { $handler, $hook, $order },
         commands {None, None, None, None, }
      );
   };

   (
      $name:ident,
      $mod_name:expr,
      commands {
         $create_dir_config:expr,
         $merge_dir_config:expr,
         $create_server_config:expr,
         $merge_server_config:expr,
         $($cmd:expr);*
      }
   ) => {
      interpolate_idents! {
         apache2_module!(
            $name,
            $mod_name,
            handlers {
               [$name _handler], handler, $crate::HookOrder::MIDDLE
            },
            commands {
               $create_dir_config,
               $merge_dir_config,
               $create_server_config,
               $merge_server_config,
               $($cmd);*
            }
         );
      }
   };

   (
      $name:ident, $mod_name:expr,
      handlers { $handler:ident, $hook:ident, $order:expr },
      commands {
         $create_dir_config:expr,
         $merge_dir_config:expr,
         $create_server_config:expr,
         $merge_server_config:expr,
         $($cmd:expr);*
      }
   ) => {
      interpolate_idents! {
         DECLARE_COMMAND_ARRAY!([$name _cmds], { $($cmd);* });

         DECLARE_MODULE!(
            [$name _module],
            $mod_name,
            $create_dir_config,
            $merge_dir_config,
            $create_server_config,
            $merge_server_config,
            &[$name _cmds],
            Some([$name _hooks])
         );

         extern "C" fn [$name _hooks](_: *mut $crate::ffi::apr_pool_t) {
            unsafe {
               $crate::ffi::[ap_hook_ $hook](
                  Some([c_ $name _handler]),
                  std::ptr::null(),
                  std::ptr::null(),
                  $order.into()
               );
            }
         }
      }


      #[no_mangle]
      interpolate_idents! {
         pub extern "C" fn [c_ $name _handler](r: *mut $crate::ffi::request_rec) -> $crate::c_int {
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
}


#[macro_export]
macro_rules! DECLARE_MODULE {
   (
      $module:ident,
      $mod_name:expr,
      $create_dir_config:expr,
      $merge_dir_config:expr,
      $create_server_config:expr,
      $merge_server_config:expr,
      $cmds:expr,
      $register_hooks:expr
   ) => {
      #[allow(unused_unsafe)]
      #[no_mangle]
      pub static mut $module: $crate::ffi::module = $crate::ffi::module {
         version: $crate::ffi::MODULE_MAGIC_NUMBER_MAJOR,
         minor_version: $crate::ffi::MODULE_MAGIC_NUMBER_MINOR,
         module_index: -1,
         name: $mod_name as *const u8 as *const $crate::c_char,
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
macro_rules! DECLARE_COMMAND_REC {
   (
      $name:expr,
      $func:expr,
      $cmd_data:expr,
      $req_override:expr,
      $args_how:expr,
      $errmsg:expr
   ) => {
      $crate::ffi::command_rec {
         name: $name as *const u8 as *const $crate::c_char,
         func: $crate::ffi::cmd_func {
            _bindgen_data_: [$func as u64]
         },
         cmd_data: $cmd_data as *mut $crate::c_void,
         req_override: $req_override,
         args_how: $args_how,
         errmsg: $errmsg as *const u8 as *const $crate::c_char
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
   ($name:expr, $func:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, 0, $req_override, $crate::ffi::NO_ARGS, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_RAW_ARGS {
   ($name:expr, $func:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, 0, $req_override, $crate::ffi::RAW_ARGS, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE_ARGV {
   ($name:expr, $func:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, 0, $req_override, $crate::ffi::TAKE_ARGV, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE1 {
   ($name:expr, $func:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, 0, $req_override, $crate::ffi::TAKE1, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_ITERATE {
   ($name:expr, $func:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, 0, $req_override, $crate::ffi::ITERATE, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE2 {
   ($name:expr, $func:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, 0, $req_override, $crate::ffi::TAKE2, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE12 {
   ($name:expr, $func:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, 0, $req_override, $crate::ffi::TAKE12, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_ITERATE2 {
   ($name:expr, $func:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, 0, $req_override, $crate::ffi::ITERATE2, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE13 {
   ($name:expr, $func:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, 0, $req_override, $crate::ffi::TAKE13, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE23 {
   ($name:expr, $func:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, 0, $req_override, $crate::ffi::TAKE23, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE123 {
   ($name:expr, $func:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, 0, $req_override, $crate::ffi::TAKE123, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_TAKE3 {
   ($name:expr, $func:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, 0, $req_override, $crate::ffi::TAKE3, $errmsg)
   }
}

#[macro_export]
macro_rules! AP_INIT_FLAG {
   ($name:expr, $func:expr, $req_override:expr, $errmsg:expr) => {
      DECLARE_COMMAND_REC!($name, $func, 0, $req_override, $crate::ffi::FLAG, $errmsg)
   }
}

#[macro_export]
macro_rules! DECLARE_COMMAND_ARRAY {
   ($cmds_name:ident, $cmd_count:expr, { $($cmd:expr);* }) => {
      #[no_mangle]
      pub static mut $cmds_name: [$crate::ffi::command_rec; $cmd_count] = [
         $($cmd),*,
         NULL_COMMAND_REC!()
      ];
   };

   ($cmds_name:ident, {}) => {
      DECLARE_COMMAND_ARRAY!($cmds_name, 1, {});
   };

   ($cmds_name:ident, { $cmd1:expr }) => {
      DECLARE_COMMAND_ARRAY!($cmds_name, 2, { $cmd1 });
   };

   ($cmds_name:ident, { $cmd1:expr; $cmd2:expr }) => {
      DECLARE_COMMAND_ARRAY!($cmds_name, 3, { $cmd1; $cmd2 });
   };

   ($cmds_name:ident, { $cmd1:expr; $cmd2:expr; $cmd3:expr }) => {
      DECLARE_COMMAND_ARRAY!($cmds_name, 4, { $cmd1; $cmd2; $cmd3 });
   };

   ($cmds_name:ident, { $cmd1:expr; $cmd2:expr; $cmd3:expr; $cmd4:expr }) => {
      DECLARE_COMMAND_ARRAY!($cmds_name, 5, { $cmd1; $cmd2; $cmd3; $cmd4 });
   };

   ($cmds_name:ident, { $cmd1:expr; $cmd2:expr; $cmd3:expr; $cmd4:expr; $cmd5:expr }) => {
      DECLARE_COMMAND_ARRAY!($cmds_name, 6, { $cmd1; $cmd2; $cmd3; $cmd4; $cmd5 });
   };

   ($cmds_name:ident, { $cmd1:expr; $cmd2:expr; $cmd3:expr; $cmd4:expr; $cmd5:expr; $cmd6:expr }) => {
      DECLARE_COMMAND_ARRAY!($cmds_name, 7, { $cmd1; $cmd2; $cmd3; $cmd4; $cmd5; $cmd6 });
   };

   ($cmds_name:ident, { $cmd1:expr; $cmd2:expr; $cmd3:expr; $cmd4:expr; $cmd5:expr; $cmd6:expr; $cmd7:expr }) => {
      DECLARE_COMMAND_ARRAY!($cmds_name, 8, { $cmd1; $cmd2; $cmd3; $cmd4; $cmd5; $cmd6; $cmd7 });
   };

   ($cmds_name:ident, { $cmd1:expr; $cmd2:expr; $cmd3:expr; $cmd4:expr; $cmd5:expr; $cmd6:expr; $cmd7:expr; $cmd8:expr }) => {
      DECLARE_COMMAND_ARRAY!($cmds_name, 9, { $cmd1; $cmd2; $cmd3; $cmd4; $cmd5; $cmd6; $cmd7; $cmd8 });
   };
}


pub type StringType<'a> = &'a str;
pub type CStringType = *const c_char;

pub type BoolType = bool;
pub type CBoolType = c_int;


#[macro_export]
macro_rules! new_module {
   ($name:ident, $mod_name:expr, config $config:tt) => {
      interpolate_idents! {
         _declare_config_struct!($name, $config);

         _declare_directives!([$name _directives], $config);

         _declare_create_server_config!($name, $config);

         DECLARE_MODULE!(
            [$name _module],
            $mod_name,
            _extract_dir_config_name!($config),
            None,
            _extract_server_config_name!($config),
            None,
            &[$name _directives],
            Some([$name _hooks])
         );
      }

      #[no_mangle]
      interpolate_idents! {
         extern "C" fn [$name _hooks](_: *mut $crate::ffi::apr_pool_t) {
            unsafe {
               $crate::ffi::ap_hook_handler(
                  Some([c_ $name _handler]),
                  std::ptr::null(),
                  std::ptr::null(),
                  $crate::HookOrder::MIDDLE.into()
               );
            }
         }
      }

      #[no_mangle]
      interpolate_idents! {
         pub extern "C" fn [c_ $name _handler](r: *mut $crate::ffi::request_rec) -> $crate::c_int {
            match $crate::httpd::Request::from_raw_ptr(r) {
               Err(_) => $crate::httpd::Status::DECLINED.into(),
               Ok(mut request) => match [$name _handler](&mut request) {
                  Ok(status) => status,
                  Err(_) => $crate::httpd::Status::HTTP_INTERNAL_SERVER_ERROR
               }.into()
            }
         }
      }
   }
}


#[macro_export]
macro_rules! _declare_config_struct {
   ($name:ident, { server $server:tt, $directives:tt }) => {
      _declare_config_struct_from_server!($name, $server);
   };

   ($name:ident, { directory $directory:tt, server $server:tt, $directives:tt }) => {
      _declare_config_struct_from_server!($name, $server);

      _declare_config_struct_from_directory!($name, $directory);
   };

   ($name:ident, { directory $directory:tt, $directives:tt }) => {
      _declare_config_struct_from_directory!($name, $directory);
   }
}


#[macro_export]
macro_rules! _declare_config_struct_from_server {
   ($name:ident, { $config_struct:ident $fields:tt, $create_server_config:ident }) => {
      _declare_config_struct_impl!($name, $config_struct $fields);

      _declare_get_module_config!($name, $config_struct, get_server_config);
   }
}


#[macro_export]
macro_rules! _declare_config_struct_from_directory {
   ($name:ident, { $config_struct:ident $fields:tt, $create_dir_config:ident }) => {
      _declare_config_struct_impl!($name, $config_struct $fields);

      _declare_get_module_config!($name, $config_struct, get_directory_config);
   }
}


#[macro_export]
macro_rules! _declare_config_struct_impl {
   ($name:ident, $struct_name:ident { $($field_name:ident: $field_type:ident),* }) => {
      #[repr(C)]
      interpolate_idents! {
         pub struct [C $struct_name] {
            $(
               pub $field_name: $crate::[C $field_type]
            ),*
         }
      }

      pub struct $struct_name<'a> {
         pub raw: &'a mut <$struct_name<'a> as $crate::CType>::c_type,
         pub pool: *mut $crate::ffi::apr_pool_t
      }

      impl<'a> $struct_name<'a> {
         pub fn new(pool: &mut Pool) -> Result<Self, ()> {
            let c_config = unsafe {
               $crate::ffi::apr_pcalloc(
                  pool.raw,
                  std::mem::size_of::<<$struct_name<'a> as $crate::CType>::c_type>() as $crate::ffi::apr_size_t
               ) as *mut <$struct_name<'a> as $crate::CType>::c_type
            };

            $struct_name::from_raw_ptr(pool, c_config)
         }

         pub fn from_raw_ptr(
            pool: &mut Pool,
            ptr: *mut <$struct_name<'a> as $crate::CType>::c_type
         ) -> Result<Self, ()> {
            if ptr.is_null() {
               Err(())
            } else {
               let raw: &mut <$struct_name<'a> as $crate::CType>::c_type = unsafe { &mut *ptr };
               Ok(
                  $struct_name {
                     raw: raw,
                     pool: pool.raw
                  }
               )
            }
         }

         $(
            _declare_config_wrapper_method!($field_name, $field_type);
         )*
      }

      interpolate_idents! {
         impl<'a> $crate::CType for $struct_name<'a> {
            type c_type = [C $struct_name];
         }
      }
   }
}


#[macro_export]
macro_rules! _declare_get_module_config {
   ($name:ident, $struct_name:ident, $get_config_fn:ident) => {
      interpolate_idents! {
         pub fn $get_config_fn<'a>(
            pool: &mut $crate::Pool,
            conf_vector: &$crate::ConfVector
         ) -> $struct_name<'a> {
            let config = unsafe {
               $crate::ffi::ap_get_module_config(conf_vector.raw, &[$name _module]) as *mut [C $struct_name]
            };

            $struct_name::from_raw_ptr(pool, config).unwrap()
         }
      }
   }
}


#[macro_export]
macro_rules! _declare_config_wrapper_method {
   ($field_name:ident, StringType) => {
      pub fn $field_name(&self) -> Result<StringType, ()> {
         $crate::from_char_ptr(self.raw.$field_name)
      }

      interpolate_idents! {
         pub fn [set_ $field_name](&mut self, value: StringType) {
            self.raw.$field_name = $crate::ffi::strdup(self.pool, value);
         }
      }
   };

   ($field_name:ident, BoolType) => {
      pub fn $field_name(&self) -> Result<BoolType, ()> {
         Ok(self.raw.$field_name != 0)
      }

      interpolate_idents! {
         pub fn [set_ $field_name](&mut self, value: BoolType) {
            match value {
               true => { self.raw.$field_name = 1; },
               false => { self.raw.$field_name = 0; },
            }
         }
      }
   }
}


#[macro_export]
macro_rules! _declare_directives {
   ($directives_name:ident, { server $server:tt, $directives:tt }) => {
      _declare_directives_impl!($directives_name, $directives, {});
   };

   ($directives_name:ident, { directory $directory:tt, server $server:tt, $directives:tt }) => {
      _declare_directives_impl!($directives_name, $directives, $directory);
   };

   ($directives_name:ident, { directory $directory:tt, $directives:tt }) => {
      _declare_directives_impl!($directives_name, $directives, $directory);
   }
}


#[macro_export]
macro_rules! _declare_directives_impl {
   ($directives_name:ident, $directives:tt, $directory:tt) => {
      _declare_directive_array!($directives_name, $directives);

      _declare_directive_wrappers!($directives, $directory);
   }
}


#[macro_export]
macro_rules! _extract_server_config_name {
   ({ server $server:tt, $directives:tt }) => {
      _extract_server_config_name_from_server!($server)
   };

   ({ directory $directory:tt, server $server:tt, $directives:tt }) => {
      _extract_server_config_name_from_server!($server)
   };

   ({ directory $directory:tt, $directives:tt }) => {
      None
   }
}


#[macro_export]
macro_rules! _extract_server_config_name_from_server {
   ({ $config_struct:ident $fields:tt, $create_server_config:ident }) => {
      interpolate_idents! {
         Some([c_ $create_server_config])
      }
   }
}


#[macro_export]
macro_rules! _extract_dir_config_name {
   ({ server $server:tt, $directives:tt }) => {
      None
   };

   ({ directory $directory:tt, server $server:tt, $directives:tt }) => {
      _extract_dir_config_name_from_directory!($directory)
   };

   ({ directory $directory:tt, $directives:tt }) => {
      _extract_dir_config_name_from_directory!($directory)
   }
}


#[macro_export]
macro_rules! _extract_dir_config_name_from_directory {
   ({ $config_struct:ident $fields:tt, $create_dir_config:ident }) => {
      interpolate_idents! {
         Some([c_ $create_dir_config])
      }
   }
}


#[macro_export]
macro_rules! _declare_create_server_config {
   ($name:ident, { server $server:tt, $directives:tt }) => {
      _declare_create_server_config_from_server!($name, $server);
   };

   ($name:ident, { directory $directory:tt, server $server:tt, $directives:tt }) => {
      _declare_create_server_config_from_server!($name, $server);

      _declare_create_dir_config_from_directory!($name, $directory);
   };

   ($name:ident, { directory $directory:tt, $directives:tt }) => {
      _declare_create_dir_config_from_directory!($name, $directory);
   }
}


#[macro_export]
macro_rules! _declare_create_server_config_from_server {
   ($name:ident, { $config_struct:ident $fields:tt, $create_server_config:ident }) => {
      _declare_create_server_config_impl!($name, $config_struct, $create_server_config);
   }
}


#[macro_export]
macro_rules! _declare_create_server_config_impl {
   ($name:ident, $config_struct:ident, $create_server_config:ident) => {
      #[no_mangle]
      interpolate_idents! {
         extern "C" fn [c_ $create_server_config](
            p: *mut $crate::ffi::apr_pool_t,
            _: *mut $crate::ffi::server_rec
         ) -> *mut $crate::c_void {
            let mut pool = Pool::from_raw_ptr(p).unwrap();

            let config = $create_server_config(&mut pool);

            config.raw as *mut [C $config_struct] as *mut $crate::c_void
         }
      }
   }
}


#[macro_export]
macro_rules! _declare_create_dir_config_from_directory {
   ($name:ident, { $config_struct:ident $fields:tt, $create_dir_config:ident }) => {
      _declare_create_dir_config_impl!($name, $config_struct, $create_dir_config);
   }
}


#[macro_export]
macro_rules! _declare_create_dir_config_impl {
   ($name:ident, $config_struct:ident, $create_dir_config:ident) => {
      #[no_mangle]
      interpolate_idents! {
         extern "C" fn [c_ $create_dir_config](
            p: *mut $crate::ffi::apr_pool_t,
            dir: *mut $crate::c_char
         ) -> *mut $crate::c_void {
            let mut pool = Pool::from_raw_ptr(p).unwrap();
            let directory = $crate::from_char_ptr(dir).unwrap();

            let config = $create_dir_config(&mut pool, &directory);

            config.raw as *mut [C $config_struct] as *mut $crate::c_void
         }
      }
   }
}


#[macro_export]
macro_rules! _declare_directive_array {
   ($directives_name:ident, $cmd_count:expr, [ $($cmd:tt),* ]) => {
      #[no_mangle]
      pub static mut $directives_name: [$crate::ffi::command_rec; $cmd_count] = [
         $( _declare_command_rec!($cmd) ),*,
         NULL_COMMAND_REC!()
      ];
   };

   ($directives_name:ident, []) => {
      _declare_directive_array!($directives_name, 1, []);
   };

   ($directives_name:ident, [ $cmd1:tt ]) => {
      _declare_directive_array!($directives_name, 2, [ $cmd1 ]);
   };

   ($directives_name:ident, [ $cmd1:tt, $cmd2:tt ]) => {
      _declare_directive_array!($directives_name, 3, [ $cmd1, $cmd2 ]);
   };

   ($directives_name:ident, [ $cmd1:tt, $cmd2:tt, $cmd3:tt ]) => {
      _declare_directive_array!($directives_name, 4, [ $cmd1, $cmd2, $cmd3 ]);
   };

   ($directives_name:ident, [ $cmd1:tt, $cmd2:tt, $cmd3:tt, $cmd4:tt ]) => {
      _declare_directive_array!($directives_name, 5, [ $cmd1, $cmd2, $cmd3, $cmd4 ]);
   };

   ($directives_name:ident, [ $cmd1:tt, $cmd2:tt, $cmd3:tt, $cmd4:tt, $cmd5:tt ]) => {
      _declare_directive_array!($directives_name, 6, [ $cmd1, $cmd2, $cmd3, $cmd4, $cmd5 ]);
   };

   ($directives_name:ident, [ $cmd1:tt, $cmd2:tt, $cmd3:tt, $cmd4:tt, $cmd5:tt, $cmd6:tt ]) => {
      _declare_directive_array!($directives_name, 7, [ $cmd1, $cmd2, $cmd3, $cmd4, $cmd5, $cmd6 ]);
   };

   ($directives_name:ident, [ $cmd1:tt, $cmd2:tt, $cmd3:tt, $cmd4:tt, $cmd5:tt, $cmd6:tt, $cmd7:tt ]) => {
      _declare_directive_array!($directives_name, 8, [ $cmd1, $cmd2, $cmd3, $cmd4, $cmd5, $cmd6, $cmd7 ]);
   };

   ($directives_name:ident, [ $cmd1:tt, $cmd2:tt, $cmd3:tt, $cmd4:tt, $cmd5:tt, $cmd6:tt, $cmd7:tt, $cmd8:tt ]) => {
      _declare_directive_array!($directives_name, 9, [ $cmd1, $cmd2, $cmd3, $cmd4, $cmd5, $cmd6, $cmd7, $cmd8 ]);
   }
}


#[macro_export]
macro_rules! _declare_command_rec {
   (($args_how:ident, $name:expr, $func:ident, $req_override:expr, $errmsg:expr)) => {
      interpolate_idents! {
         _declare_c_command_rec!($args_how, $name, [c_ $func], $req_override, $errmsg)
      }
   }
}


#[macro_export]
macro_rules! _declare_c_command_rec {
   ($args_how:ident, $name:expr, $cfunc:ident, $req_override:expr, $errmsg:expr) => {
      $crate::ffi::command_rec {
         name: $name as *const u8 as *const $crate::c_char,
         func: $crate::ffi::cmd_func {
            _bindgen_data_: [$cfunc as u64]
         },
         cmd_data: 0 as *mut $crate::c_void,
         req_override: $req_override,
         args_how: $crate::$args_how,
         errmsg: $errmsg as *const u8 as *const $crate::c_char
      }
   }
}


#[macro_export]
macro_rules! _declare_directive_wrappers {
   ([ $(( $args_how:ident, $name:expr, $func:ident, $req_override:expr, $errmsg:expr )),* ], $directory:tt) => {
      $(
         _declare_directive_c_wrapper!($args_how, $func, $directory);
      )*
   }
}


#[macro_export]
macro_rules! _declare_directive_c_wrapper {
   (FLAG, $func:ident, $directory:tt) => {
      #[no_mangle]
      interpolate_idents! {
         extern "C" fn [c_ $func](
            parms: *mut $crate::ffi::cmd_parms,
            mconfig: *mut $crate::c_void,
            on: $crate::c_int
         ) -> *const $crate::c_char {
            let mut wrapper = CmdParms::from_raw_ptr(parms).unwrap();
            let mut pool = Pool::from_raw_ptr(unsafe { (*parms).pool }).unwrap();

            _call_config_wrapper!($func, &mut wrapper, &mut pool, mconfig, on != 0, $directory).unwrap();

            std::ptr::null()
         }
      }
   };

   (TAKE1, $func:ident, $directory:tt) => {
      #[no_mangle]
      interpolate_idents! {
         extern "C" fn [c_ $func](
            parms: *mut $crate::ffi::cmd_parms,
            mconfig: *mut $crate::c_void,
            w: *const $crate::c_char
         ) -> *const $crate::c_char {
            let mut wrapper = CmdParms::from_raw_ptr(parms).unwrap();
            let mut pool = Pool::from_raw_ptr(unsafe { (*parms).pool }).unwrap();

            _call_config_wrapper!($func, &mut wrapper, &mut pool, mconfig, $crate::from_char_ptr(w).unwrap(), $directory).unwrap();

            std::ptr::null()
         }
      }
   }
}


#[macro_export]
macro_rules! _call_config_wrapper {
   ($func:ident, $parms:expr, $pool:expr, $mconfig:expr, $arg1:expr, {}) => {
      $func($parms, $arg1)
   };

   ($func:ident, $parms:expr, $pool:expr, $mconfig:expr, $arg1:expr, { $config_struct:ident $fields:tt, $create_dir_config:ident }) => {
      interpolate_idents! {
         $func($parms, $config_struct::from_raw_ptr($pool, $mconfig as *mut [C $config_struct]).ok(), $arg1)
      }
   }
}
