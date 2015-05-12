#![allow(dead_code)]
#![allow(unused_imports)]

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
use apache2::wrapper::{Wrapper, CType, from_char_ptr};
use apache2::apr::Pool;



type CStringType = *const c_char;
type StringType<'a> = &'a str;


macro_rules! config_struct {
   ($name:ident { $( $field_name:ident: $field_type:ident ),* }) => {
      #[repr(C)]
      interpolate_idents! {
         config_define_c_struct!(
            [C $name] {
               $(
                  $field_name: [C $field_type]
               ),*
            }
         );
      }

      struct $name<'a> {
         pub raw: &'a mut <$name<'a> as CType>::c_type,
         pub pool: *mut ffi::apr_pool_t
      }

      impl<'a> $name<'a> {
         pub fn new(pool: &mut Pool) -> Result<Self, ()> {
            let c_config = unsafe {
               ffi::apr_pcalloc(
                  pool.raw,
                  mem::size_of::<<$name<'a> as CType>::c_type>() as ffi::apr_size_t
               ) as *mut <$name<'a> as CType>::c_type
            };

            $name::from_raw_ptr(c_config, pool.raw)
         }

         pub fn from_raw_ptr(ptr: *mut <$name<'a> as CType>::c_type, pool: *mut ffi::apr_pool_t) -> Result<Self, ()> {
            if ptr.is_null() {
               Err(())
            } else {
               let raw: &mut <$name<'a> as CType>::c_type = unsafe { &mut *ptr };
               Ok(
                  $name {
                     raw: raw,
                     pool: pool
                  }
               )
            }
         }

         $(
            config_wrapper_method!($field_name, $field_type);
         )*
      }

      interpolate_idents! {
         impl<'a> CType for $name<'a> {
            type c_type = [C $name];
         }
      }
   }
}

macro_rules! config_define_c_struct {
   ($c_name:ident { $( $field_name:ident: $field_type:ident ),* } ) => {
      #[repr(C)]
      struct $c_name {
         $(
            pub $field_name: $field_type,
         )*
      }
   }
}

macro_rules! config_wrapper_method {
   ($field_name:ident, StringType) => {
      pub fn $field_name(&self) -> Result<StringType, ()> {
         from_char_ptr(self.raw.$field_name)
      }

      interpolate_idents! {
         pub fn [set_ $field_name](&mut self, value: StringType) {
            self.raw.$field_name = ffi::strdup(self.pool, value);
         }
      }
   }
}

#[allow(unused_variables)]
pub extern "C" fn first_cmd(parms: *mut ffi::cmd_parms, p: *mut c_void, w: *const c_char) -> *const c_char {
   let config = unsafe { ffi::ap_get_module_config((*(*parms).server).module_config, &conf_module) as *mut CExampleConfig };

   let mut example_config = ExampleConfig::from_raw_ptr(config, unsafe { (*parms).pool }).unwrap();

   example_config.set_first_cmd(from_char_ptr(w).unwrap());

   std::ptr::null()
}


#[allow(unused_variables)]
pub extern "C" fn second_cmd(parms: *mut ffi::cmd_parms, p: *mut c_void, w: *const c_char) -> *const c_char {
   let config = unsafe { ffi::ap_get_module_config((*(*parms).server).module_config, &conf_module) as *mut CExampleConfig };

   let mut example_config = ExampleConfig::from_raw_ptr(config, unsafe { (*parms).pool }).unwrap();

   example_config.set_second_cmd(from_char_ptr(w).unwrap());

   std::ptr::null()
}


#[allow(unused_variables)]
pub extern "C" fn c_create_server_config(p: *mut ffi::apr_pool_t, s: *mut ffi::server_rec) -> *mut c_void {
   let mut pool = Pool::from_raw_ptr(p).unwrap();
   let config = create_server_config(&mut pool);

   config.raw as *mut CExampleConfig as *mut c_void
}


fn create_server_config<'a>(pool: &mut Pool) -> ExampleConfig<'a> {
   ExampleConfig::new(pool).unwrap()
}

config_struct!(
   ExampleConfig {
      first_cmd: StringType,
      second_cmd: StringType
   }
);


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


   let config = unsafe { ffi::ap_get_module_config(try!(try!(r.server()).module_config()).raw, &conf_module) as *mut CExampleConfig };

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
