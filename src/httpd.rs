
pub mod raw {
   use libc::{c_int};

   #[repr(C)]
   pub struct process_rec;

   #[repr(C)]
   pub struct request_rec;

   #[repr(C)]
   pub struct server_rec;

   pub const OK:        c_int = 0;
   pub const DECLINED:  c_int = -1;
   pub const DONE:      c_int = -2;
   pub const SUSPENDED: c_int = -3;
}
