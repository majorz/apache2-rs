pub mod raw {
   #![allow(non_camel_case_types)]

   use libc::{c_void, c_char, c_int, c_ulong};

   use apr::raw::{apr_pool_t};

   use httpd::raw::{process_rec, server_rec};

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
