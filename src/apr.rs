pub mod raw {
   #![allow(non_camel_case_types)]

   use libc::{c_char, c_uchar, c_short, c_ushort, c_int, c_uint, c_long, c_ulong};

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

   extern "C" {
      pub fn apr_table_get(t: *const apr_table_t, key: *const c_char) -> *const c_char;
      pub fn apr_table_set(t: *mut apr_table_t, key: *const c_char, val: *const c_char) -> ();
      pub fn apr_pstrmemdup(p: *mut apr_pool_t, s: *const c_char, n: apr_size_t) -> *mut c_char;
      pub fn apr_version_string() -> *const c_char;
      pub fn apu_version_string() -> *const c_char;
   }

   pub fn dup_c_str<T: Into<Vec<u8>>>(pool: *mut apr_pool_t, data: T) -> *mut c_char {
      let bytes = data.into();

      unsafe {
         apr_pstrmemdup(
            pool,
            bytes.as_ptr() as *const c_char,
            bytes.len() as apr_size_t
         )
      }
   }

   #[repr(C)]
   pub struct apr_array_header_t;

   #[repr(C)]
   pub struct apr_bucket_alloc_t;

   #[repr(C)]
   pub struct apr_bucket_brigade;

   #[repr(C)]
   pub struct apr_finfo_t;

   #[repr(C)]
   pub struct apr_pool_t;

   #[repr(C)]
   pub struct apr_sockaddr_t;

   #[repr(C)]
   pub struct apr_table_t;

   #[repr(C)]
   pub struct apr_thread_mutex_t;

   #[repr(C)]
   pub struct apr_thread_t;

   #[repr(C)]
   pub struct apr_uri_t;

   pub type apr_byte_t = c_uchar;
   pub type apr_int16_t = c_short;
   pub type apr_uint16_t = c_ushort;
   pub type apr_int32_t = c_int;
   pub type apr_uint32_t = c_uint;
   pub type apr_int64_t = c_long;
   pub type apr_uint64_t = c_ulong;
   pub type apr_size_t = c_ulong;
   pub type apr_ssize_t = c_long;
   pub type apr_off_t = c_long;
   pub type apr_socklen_t = c_uint;
   pub type apr_ino_t = c_ulong;
   pub type apr_uintptr_t = apr_uint64_t;
   pub type apr_status_t = c_int;
   pub type apr_signum_t = c_int;
   pub type apr_time_t = apr_int64_t;
   pub type apr_port_t = apr_uint16_t;
}

use std::ffi::CString;

use wrapper::{Wrapper, c_str_value};


pub type AprTable<'a> = Wrapper<'a, raw::apr_table_t>;


impl<'a> AprTable<'a> {
   pub fn get(&self, key: &'static str) -> Option<&'a str> {
      let p: *const raw::apr_table_t = self.raw;
      c_str_value(
         unsafe { raw::apr_table_get(p, CString::new(key).unwrap().as_ptr()) }
      )
   }

   pub fn set(&mut self, key: &'static str, val: &'static str) {
      let p: *mut raw::apr_table_t = self.raw;
      unsafe {
         raw::apr_table_set(
            p,
            CString::new(key).unwrap().as_ptr(),
            CString::new(val).unwrap().as_ptr()
         )
      };
   }
}

pub fn apr_version_string<'a>() -> Option<&'a str> {
   c_str_value(
      unsafe { raw::apr_version_string() }
   )
}

pub fn apu_version_string<'a>() -> Option<&'a str> {
   c_str_value(
      unsafe { raw::apu_version_string() }
   )
}
