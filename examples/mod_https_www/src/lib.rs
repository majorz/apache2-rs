#![feature(plugin)]
#![feature(collections)]
#![plugin(interpolate_idents)]

#[macro_use]
extern crate apache2;

use apache2::{HookOrder, Request, Status};

apache2_module!(https_www, b"mod_https_www\0", handlers {
   (https_www_handler, translate_name, HookOrder::MIDDLE)
});


fn https_www_handler(r: &mut Request) -> Result<Status, ()> {
   let scheme = r.http_scheme().unwrap();
   let hostname = r.hostname().unwrap();

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

   let uri = r.unparsed_uri().unwrap();

   let location = format!("https://{}{}", full_hostname, uri);

   try!(
      r.headers_out().unwrap().set("Location", location)
   );

   r.set_status(apache2::Status::HTTP_MOVED_PERMANENTLY);

   Ok(Status::DONE)
}
