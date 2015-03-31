pub mod raw {
   use libc::{c_int, c_ulong};

   pub const MODULE_MAGIC_COOKIE: c_ulong = 0x41503234u64; /* "AP24" */

   pub const MODULE_MAGIC_NUMBER_MAJOR: c_int = 20120211;
   pub const MODULE_MAGIC_NUMBER_MINOR: c_int = 36;
}
