#![feature(libc)]

extern crate libc;

#[macro_use]
extern crate apache2;

use apache2::{Request, Status};

apache2_module!(comingsoon_module, comingsoon_handler, b"mod_comingsoon\0");


fn comingsoon_handler(r: &Request) -> Status {
   if r.handler().unwrap() != "comingsoon" || r.uri().unwrap() != "/" {
      return Status::DECLINED
   }

   r.set_content_type("text/html");

   r.write(
      "<!doctype html>
      <html>
      <head>
         <meta charset=\"utf-8\">
         <title>Coming Soon...</title>
      </head>
      <body>
         <h1>Coming Soon...</h1>
      </body>
      </html>"
   );

   Status::OK
}
