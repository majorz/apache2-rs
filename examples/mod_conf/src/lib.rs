#![feature(plugin)]
#![plugin(interpolate_idents)]

extern crate libc;

#[macro_use]
extern crate apache2;

#[macro_use]
extern crate lazy_static;

use std::mem;
use libc::{c_void, c_char};
use apache2::{Request, Status, Server, ffi};
use apache2::wrapper::{Wrapper, from_char_ptr};


macro_rules! config_struct {
   ($name:ident { $( $fieldname:ident: $ctype:ty ),* }) => {
      #[repr(C)]
      interpolate_idents! {
         struct [$name C] {
            $(
               pub $fieldname: $ctype,
            )*
         }

         type $name<'a> = apache2::wrapper::Wrapper<'a, [$name C]>;
      }
   }
}


config_struct!(
   ExampleConfig {
      first_cmd: *const c_char,
      second_cmd: *const c_char
   }
);


#[allow(unused_variables)]
pub extern "C" fn first_cmd(parms: *mut ffi::cmd_parms, p: *mut c_void, w: *const c_char) -> *const c_char {
   let config = unsafe { ffi::ap_get_module_config((*(*parms).server).module_config, &conf_module) as *mut ExampleConfigC };

   unsafe { (*config).first_cmd = w };

   std::ptr::null()
}


#[allow(unused_variables)]
pub extern "C" fn second_cmd(parms: *mut ffi::cmd_parms, p: *mut c_void, w: *const c_char) -> *const c_char {
   let config = unsafe { ffi::ap_get_module_config((*(*parms).server).module_config, &conf_module) as *mut ExampleConfigC };

   unsafe { (*config).second_cmd = w };

   std::ptr::null()
}


#[allow(unused_variables)]
pub extern "C" fn c_create_server_config(p: *mut ffi::apr_pool_t, s: *mut ffi::server_rec) -> *mut c_void {
   let config: *mut ExampleConfigC = unsafe {
      ffi::apr_pcalloc(p, mem::size_of::<ExampleConfigC>() as ffi::apr_size_t) as *mut ExampleConfigC
   };

   config as *mut c_void
}


//fn create_server_config(pool: &mut Pool, _: &Server) -> ExampleConfigC {

//}

apache2_module!(conf, b"mod_conf\0", commands {
   None,
   None,
   Some(c_create_server_config),
   None,
   AP_INIT_TAKE1!(b"FirstCmd\0", first_cmd, apache2::ffi::RSRC_CONF, b"First cmd description\0");
   AP_INIT_TAKE1!(b"SecondCmd\0", second_cmd, apache2::ffi::RSRC_CONF, b"Second cmd description\0")
});

fn unwrap_str<'a>(wrapped: Result<&'a str, ()>) -> &'a str {
   wrapped.unwrap_or("--")
}

fn conf_handler(r: &mut Request) -> Result<Status, ()> {
   if try!(r.handler()) != "conf" {
      return Ok(Status::DECLINED)
   }


   let config = unsafe { ffi::ap_get_module_config(try!(try!(r.server()).module_config()).raw, &conf_module) as *mut ExampleConfigC };

   r.set_content_type("text/plain; charset=utf-8");

   try!(r.write("FIRST CMD: "));

   let first_cmd = unwrap_str(from_char_ptr(unsafe { (*config).first_cmd }));
   try!(r.write(first_cmd));

   try!(r.write("\n"));

   try!(r.write("SECOND CMD: "));

   let second_cmd = unwrap_str(from_char_ptr(unsafe { (*config).second_cmd }));
   try!(r.write(second_cmd));

   try!(r.write("\n"));

   Ok(Status::OK)
}
