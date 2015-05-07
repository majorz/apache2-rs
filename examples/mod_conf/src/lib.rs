#![feature(plugin)]
#![plugin(interpolate_idents)]

extern crate libc;

#[macro_use]
extern crate apache2;


use libc::{c_void, c_char};
use apache2::{Request, Status, CmdParms, ffi};


fn cmd_process(parms: &mut CmdParms) {
}


#[allow(unused_variables)]
pub extern "C" fn cmd(parms: *mut ffi::cmd_parms, mconfig: *mut c_void, w: *const c_char) -> *const c_char {
   std::ptr::null()
}


#[allow(unused_variables)]
pub extern "C" fn second_cmd(parms: *mut ffi::cmd_parms, mconfig: *mut c_void, w: *const c_char) -> *const c_char {
   std::ptr::null()
}

apache2_module!(conf, b"mod_conf\0", ap_hook_handler, apache2::HookOrder::MIDDLE, [
   AP_INIT_TAKE1!(b"SomeCmd\0", cmd, apache2::ffi::RSRC_CONF, b"Error message\0");
   AP_INIT_TAKE1!(b"SecondCmd\0", second_cmd, apache2::ffi::RSRC_CONF, b"Second error message\0")
]);


fn conf_handler(r: &mut Request) -> Result<Status, ()> {
   if try!(r.handler()) != "conf" {
      return Ok(Status::DECLINED)
   }

   r.set_content_type("text/plain; charset=utf-8");

   try!(r.write("CONF: *****"));

   Ok(Status::OK)
}
