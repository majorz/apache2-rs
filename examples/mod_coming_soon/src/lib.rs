extern crate libc;

#[macro_use]
extern crate apache2;

use apache2::{Request, Status};

apache2_module!(coming_soon_handler, c_coming_soon_handler, coming_soon_module, b"mod_coming_soon\0");


fn coming_soon_handler(r: &Request) -> Status {
   if r.handler().unwrap() != "coming-soon" || r.uri().unwrap() != "/" {
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
