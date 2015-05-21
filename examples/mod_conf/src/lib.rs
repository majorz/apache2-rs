#![feature(convert)]
#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use]
extern crate apache2;

use apache2::{Request, Status, Pool, CmdParms, BoolType, StringType, RSRC_CONF, ACCESS_CONF};


apache2_module!(
   conf, b"mod_conf\0",
   config {
      directory {
         DirectoryConfig {
            dir_var: StringType
         },
         create_dir_config,
         merge_dir_config
      },
      server {
         ServerConfig {
            enabled: BoolType,
            string_var: StringType
         },
         create_server_config,
         merge_server_config
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


fn merge_server_config<'a>(pool: &mut Pool, base_conf: &'a ServerConfig, new_conf: &'a ServerConfig) -> ServerConfig<'a> {
   let mut config = create_server_config(pool);

   config.set_enabled(
      new_conf.enabled().unwrap_or(
         base_conf.enabled().unwrap_or(false)
      )
   );

   config.set_string_var(
      new_conf.string_var().unwrap_or(
         base_conf.string_var().unwrap_or("")
      )
   );

   config
}


fn create_dir_config<'a>(pool: &mut Pool, _: Option<&'a str>) -> DirectoryConfig<'a> {
   DirectoryConfig::new(pool).unwrap()
}


fn merge_dir_config<'a>(pool: &mut Pool, base_conf: &'a DirectoryConfig, new_conf: &'a DirectoryConfig) -> DirectoryConfig<'a> {
   let mut config = create_dir_config(pool, None);

   config.set_dir_var(
      format!(
         "{}{}", base_conf.dir_var().unwrap_or(""), new_conf.dir_var().unwrap_or("")
      ).as_str()
   );

   config
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


fn unwrap_str<'a>(wrapped: Result<&'a str, ()>) -> &'a str {
   wrapped.unwrap_or("--")
}


fn conf_handler(r: &mut Request) -> Result<Status, ()> {
   if try!(r.handler()) != "conf" {
      return Ok(Status::DECLINED)
   }

   let server_config = get_server_config(
      &mut try!(r.pool()),
      &try!(try!(r.server()).module_config())
   );

   r.set_content_type("text/plain; charset=utf-8");

   let enabled = try!(server_config.enabled());
   try!(r.write(format!("EnabledVar: {}\n", enabled)));

   let string_var = unwrap_str(server_config.string_var());
   try!(r.write(format!("StringVar: {}\n", string_var)));

   let dir_config = get_dir_config(
      &mut try!(r.pool()),
      &try!(r.per_dir_config())
   );

   let dir_var = unwrap_str(dir_config.dir_var());
   try!(r.write(format!("DirVar: {}\n", dir_var)));

   Ok(Status::OK)
}
