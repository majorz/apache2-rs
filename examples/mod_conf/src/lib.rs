#![feature(plugin)]
#![plugin(interpolate_idents)]

extern crate libc;

#[macro_use]
extern crate apache2;

use apache2::{Request, Status, Pool, CmdParms, BoolType, StringType, RSRC_CONF, ACCESS_CONF};


new_module!(
   conf, b"mod_conf\0",
   config {
      directory {
         DirectoryConfig {
            dir_var: StringType
         },
         create_directory_config
      },
      server {
         ServerConfig {
            enabled: BoolType,
            string_var: StringType
         },
         create_server_config
      }, [
         (FLAG, b"EnabledVar\0", enabled_var, RSRC_CONF, b"Example flag\0"),
         (TAKE1, b"StringVar\0", string_var, RSRC_CONF, b"Example string directive\0"),
         (TAKE1, b"DirVar\0", dir_var, ACCESS_CONF, b"Directory string directive\0")
      ]
   }
);


fn create_server_config<'a>(pool: &mut Pool) -> ServerConfig<'a> {
   let mut config = ServerConfig::new(pool).unwrap();

   config.set_enabled(false);

   config
}


fn create_directory_config<'a>(pool: &mut Pool, _: Option<&'a str>) -> DirectoryConfig<'a> {
   DirectoryConfig::new(pool).unwrap()
}


fn enabled_var(parms: &mut CmdParms, _: Option<DirectoryConfig>, on: bool) -> Result<(), ()> {
   let mut config = get_server_config(
      &mut try!(parms.pool()),
      &try!(try!(parms.server()).module_config())
   );

   config.set_enabled(on);

   Ok(())
}


fn string_var<'a>(parms: &mut CmdParms, _: Option<DirectoryConfig>, w: &'a str) -> Result<(), ()> {
   let mut config = get_server_config(
      &mut try!(parms.pool()),
      &try!(try!(parms.server()).module_config())
   );

   config.set_string_var(w);

   Ok(())
}


fn dir_var<'a>(_: &mut CmdParms, mconfig: Option<DirectoryConfig>, w: &'a str) -> Result<(), ()> {
   let mut config = mconfig.unwrap();

   config.set_dir_var(w);

   Ok(())
}


fn conf_handler(r: &mut Request) -> Result<Status, ()> {
   if try!(r.handler()) != "conf" {
      return Ok(Status::DECLINED)
   }

   let config = get_server_config(
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
