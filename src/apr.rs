pub mod raw {
   #![allow(non_camel_case_types)]

   use libc::{c_uchar, c_short, c_ushort, c_int, c_uint, c_long, c_ulong};

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
   pub struct apr_array_header_t;

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
}
