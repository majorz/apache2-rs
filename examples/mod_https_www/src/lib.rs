#![feature(plugin)]
#![feature(collections)]
#![plugin(interpolate_idents)]

#[macro_use]
extern crate apache2;

use apache2::{HookOrder, Request, Status};

apache2_module!(https_www, b"mod_https_www\0", ap_hook_translate_name, HookOrder::MIDDLE);


fn https_www_handler(r: &mut Request) -> Result<Status, ()> {
   let scheme = try!(r.http_scheme());
   let hostname = try!(r.hostname());

   let already_www = &hostname[..4] == "www.";
   let already_https = scheme == "https";

   if already_www && already_https {
      return Ok(Status::DECLINED);
   };

   let full_hostname = if already_www {
      String::from_str(hostname)
   } else {
      format!("www.{}", hostname)
   };

   let uri = try!(r.unparsed_uri());

   let location = format!("https://{}{}", full_hostname, uri);

   try!(
      try!(r.headers_out()).set("Location", location)
   );

   r.set_status(apache2::Status::HTTP_MOVED_PERMANENTLY);

   Ok(Status::DONE)
}
