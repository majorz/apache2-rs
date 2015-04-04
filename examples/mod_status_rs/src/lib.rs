#![feature(libc)]

extern crate libc;

#[macro_use]
extern crate apache2;

use apache2::{Request, Status, get_server_description};

apache2_module!(status_rs_module, status_rs_handler, c_status_rs_handler, b"mod_status_rs\0");


fn status_rs_handler(r: &Request) -> Status {
   if r.handler().unwrap() != "server-status-rs" {
      return Status::DECLINED
   }

   let server_description = get_server_description().unwrap();

   let conn = r.connection().unwrap();
   let client_ip = conn.client_ip().unwrap();

   r.set_content_type("text/html");

   r.write(format!(
      "<!doctype html>
      <html>
      <head>
         <meta charset=\"utf-8\">
         <title>Apache Status</title>
      </head>
      <body>
         <h1>Apache Server Status</h1>
         <p>Server Version: {}</p>
         <p>Client IP: {}</p>
      </body>
      </html>",
      server_description,
      client_ip,
   ));

   Status::OK
}
