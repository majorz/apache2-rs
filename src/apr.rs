pub mod raw {
   use libc::{c_int};

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
   pub struct apr_pool_t;
}
