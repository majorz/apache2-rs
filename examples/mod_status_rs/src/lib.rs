#![feature(libc)]

extern crate libc;

#[macro_use]
extern crate apache2;

use apache2::{Request, Status, get_server_description, show_mpm};

apache2_module!(status_rs_module, status_rs_handler, c_status_rs_handler, b"mod_status_rs\0");


fn status_rs_handler(r: &Request) -> Status {
   if r.handler().unwrap() != "server-status-rs" {
      return Status::DECLINED
   }

   let server_description = get_server_description().unwrap();

   let conn = r.connection().unwrap();
   let client_ip = conn.client_ip().unwrap();
   let local_ip = conn.local_ip().unwrap();
   let server_name = r.escape_html(r.server_name().unwrap()).unwrap();
   let mmp = show_mpm().unwrap();

   r.set_content_type("text/html");

   r.write(format!(
      "<!doctype html>
      <html>
      <head>
         <meta charset=\"utf-8\">
         <title>Apache Status</title>
      </head>
      <body>
         <h1>Apache Server Status for {} (via {})</h1>
         <p>Server Version: {}</p>
         <p>Server MPM: {}</p>
         <p>Client IP: {}</p>
      </body>
      </html>",
      server_name,
      local_ip,
      server_description,
      mmp,
      client_ip,
   ));

   Status::OK
}
