#![feature(plugin)]
#![plugin(interpolate_idents)]

extern crate libc;

#[macro_use]
extern crate apache2;

use apache2::{Request, Status, Pool, CmdParms, BoolType, StringType, RSRC_CONF};


new_module!(
   conf_new, b"mod_conf_new\0",
   config {
      server {
         ExampleConfig {
            enabled: BoolType,
            string_var: StringType
         },
         create_example_server_config
      }, [
         (FLAG, b"EnabledVar\0", enabled_var, RSRC_CONF, b"Example flag\0"),
         (TAKE1, b"StringVar\0", string_var, RSRC_CONF, b"Example string string directive\0")
      ]
   }
);


fn create_example_server_config<'a>(pool: &mut Pool) -> ExampleConfig<'a> {
   let mut config = ExampleConfig::new(pool).unwrap();

   config.set_enabled(false);

   config
}


fn enabled_var(parms: &mut CmdParms, on: bool) -> Result<(), ()> {
   let mut config = get_module_config(
      &mut try!(parms.pool()),
      &try!(try!(parms.server()).module_config())
   );

   config.set_enabled(on);

   Ok(())
}


fn string_var<'a>(parms: &mut CmdParms, w: &'a str) -> Result<(), ()> {
   let mut config = get_module_config(
      &mut try!(parms.pool()),
      &try!(try!(parms.server()).module_config())
   );

   config.set_string_var(w);

   Ok(())
}


fn conf_new_handler(r: &mut Request) -> Result<Status, ()> {
   if try!(r.handler()) != "conf" {
      return Ok(Status::DECLINED)
   }

   let config = get_module_config(
      &mut try!(r.pool()),
      &try!(try!(r.server()).module_config())
   );

   r.set_content_type("text/plain; charset=utf-8");

   let enabled = try!(config.enabled());
   try!(r.write(format!("EnabledVar: {:?}\n", enabled)));

   let string_var = try!(config.string_var());
   try!(r.write(format!("StringVar: {:?}\n", string_var)));

   Ok(Status::OK)
}
