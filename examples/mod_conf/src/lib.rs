extern crate libc;

#[macro_use]
extern crate apache2;


use libc::{c_void, c_char};
use apache2::{Request, Status, ffi};

pub const X: u32 = ffi::NONFATAL_OVERRIDE | ffi::NONFATAL_UNKNOWN;


#[allow(unused_variables)]
pub extern "C" fn cmd(parms: *mut ffi::cmd_parms, mconfig: *mut c_void, w: *const c_char) -> *const c_char {
   std::ptr::null()
}

/*
pub name: *const c_char,
pub func: cmd_func,
pub cmd_data: *mut c_void,
pub req_override: c_int,
pub args_how: cmd_how,
pub errmsg: *const c_char,
*/
#[no_mangle]
pub static mut dav_cmd: ffi::command_rec = ffi::command_rec {
   name: b"SomeCmd\0" as *const u8 as *const c_char,
   func: ffi::cmd_func {
      _bindgen_data_: [cmd as u64]
   },
   cmd_data: 0 as *mut c_void,
   req_override: 0,
   args_how: 0,
   errmsg: 0 as *const c_char
};


apache2_module!(conf_handler, c_conf_handler, conf_module, b"mod_conf\0");


fn conf_handler(r: &mut Request) -> Result<Status, ()> {
   r.set_content_type("text/plain; charset=utf-8");

   try!(r.write("CONF: *****"));

   Ok(Status::OK)
}
