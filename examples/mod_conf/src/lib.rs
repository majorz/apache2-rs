#![feature(plugin)]
#![plugin(interpolate_idents)]

extern crate libc;

#[macro_use]
extern crate apache2;

#[macro_use]
extern crate lazy_static;

use libc::{c_char};
use apache2::{Request, Status, ffi};
use apache2::wrapper::from_char_ptr;


#[repr(C)]
pub struct example_config {
   pub first_cmd: *const c_char,
   pub second_cmd: *const c_char
}


#[allow(unused_variables)]
pub extern "C" fn first_cmd(parms: *mut ffi::cmd_parms, config: *mut example_config, w: *const c_char) -> *const c_char {
   let val = from_char_ptr(w).unwrap();
   std::ptr::null()
}


#[allow(unused_variables)]
pub extern "C" fn second_cmd(parms: *mut ffi::cmd_parms, config: *mut example_config, w: *const c_char) -> *const c_char {
   let val = from_char_ptr(w).unwrap();
   std::ptr::null()
}

apache2_module!(conf, b"mod_conf\0", commands {
   None, None, None, None,
   AP_INIT_TAKE1!(b"FirstCmd\0", first_cmd, apache2::ffi::RSRC_CONF, b"First cmd description\0");
   AP_INIT_TAKE1!(b"SecondCmd\0", second_cmd, apache2::ffi::RSRC_CONF, b"Second cmd description\0")
});


fn conf_handler(r: &mut Request) -> Result<Status, ()> {
   if try!(r.handler()) != "conf" {
      return Ok(Status::DECLINED)
   }

   r.set_content_type("text/plain; charset=utf-8");

   try!(r.write("CONF: *****"));

   Ok(Status::OK)
}
