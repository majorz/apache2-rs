extern crate libc;

#[macro_use]
extern crate apache2;


use libc::{c_void, c_char};
use apache2::{Request, Status, ffi};


#[allow(unused_variables)]
pub extern "C" fn cmd(parms: *mut ffi::cmd_parms, mconfig: *mut c_void, w: *const c_char) -> *const c_char {
   std::ptr::null()
}


#[no_mangle]
pub static mut SOME_CMD: ffi::command_rec = ffi::command_rec {
   name: b"SomeCmd\0" as *const u8 as *const c_char,
   func: ffi::cmd_func {
      _bindgen_data_: [cmd as u64]
   },
   cmd_data: 0 as *mut c_void,
   req_override: apache2::ffi::RSRC_CONF,
   args_how: apache2::ffi::TAKE1,
   errmsg: b"Error message\0" as *const u8 as *const c_char
};


#[no_mangle]
pub static mut EXAMPLE_DIRECTIVES: [*const ffi::command_rec; 2] = [
   &ffi::command_rec {
      name: b"SomeCmd\0" as *const u8 as *const c_char,
      func: ffi::cmd_func {
         _bindgen_data_: [cmd as u64]
      },
      cmd_data: 0 as *mut c_void,
      req_override: apache2::ffi::RSRC_CONF,
      args_how: apache2::ffi::TAKE1,
      errmsg: b"Error message\0" as *const u8 as *const c_char
   },
   0 as *const ffi::command_rec
];


AP_DECLARE_MODULE!(
   conf_module,
   b"mod_conf\0",
   None,
   None,
   None,
   None,
   unsafe { &EXAMPLE_DIRECTIVES as *const *const ffi::command_rec },
   Some(c_module_hooks)
);


extern "C" fn c_module_hooks(_: *mut apache2::ffi::apr_pool_t) {
   unsafe {
      apache2::ffi::ap_hook_handler(
         Some(c_conf_handler),
         std::ptr::null(),
         std::ptr::null(),
         apache2::HookOrder::MIDDLE.into()
      );
   }
}


#[no_mangle]
pub extern "C" fn c_conf_handler(r: *mut apache2::ffi::request_rec) -> apache2::c_int {
   match apache2::httpd::Request::from_raw_ptr(r) {
      Err(_) => apache2::httpd::Status::DECLINED.into(),
      Ok(mut request) => match conf_handler(&mut request) {
         Ok(status) => status,
         Err(_) => apache2::httpd::Status::HTTP_INTERNAL_SERVER_ERROR
      }.into()
   }
}


fn conf_handler(r: &mut Request) -> Result<Status, ()> {
   if try!(r.handler()) != "conf" {
      return Ok(Status::DECLINED)
   }

   r.set_content_type("text/plain; charset=utf-8");

   try!(r.write("CONF: *****"));

   Ok(Status::OK)
}
