extern crate libc;

#[macro_use]
extern crate apache2;

use apache2::{Request, Status, server_banner, server_description, server_built, show_mpm,
   apr_version_string, apu_version_string};

apache2_module!(info_rs_module, info_rs_handler, c_info_rs_handler, b"mod_info_rs\0");

fn unwrap_str<'a>(option: Option<&'a str>) -> &'a str {
   match option {
      Some(val) => val,
      None => "--"
   }
}

fn info_rs_handler(r: &Request) -> Status {
   if r.handler().unwrap() != "server-info-rs" {
      return Status::DECLINED
   }

   let conn = r.connection().unwrap();

   r.set_content_type("text/html");

   r.write("<!doctype html><html><head><meta charset=\"utf-8\"><title>Apache Info</title></head><body>");

   r.write("<h1>Apache Server Information</h1>");

   let server_name = unwrap_str(
      r.escape_html(
         unwrap_str(r.server_name())
      )
   );
   let server_port = r.server_port();
   let local_ip = unwrap_str(conn.local_ip());
   r.write(format!("<p>Server: {}:{} (via {})</p>", server_name, server_port, local_ip));

   let description = unwrap_str(server_description());
   let banner = unwrap_str(server_banner());
   r.write(format!("<p>Server Description/Banner: {} / {}</p>", description, banner));

   let mmp = unwrap_str(show_mpm());
   r.write(format!("<p>Server MPM: {}</p>", mmp));

   let built = unwrap_str(server_built());
   r.write(format!("<p>Server Built: {}</p>", built));

   let apr_version = unwrap_str(apr_version_string());
   r.write(format!("<p>Server loaded APR Version: {}</p>", apr_version));

   let apu_version = unwrap_str(apu_version_string());
   r.write(format!("<p>Server loaded APU Version: {}</p>", apu_version));

   let document_root = unwrap_str(r.document_root());
   r.write(format!("<p>Document Root: {}</p>", document_root));

   r.write("<hr />");

   let client_ip = unwrap_str(conn.client_ip());
   r.write(format!("<p>Client IP: {}</p>", client_ip));

   r.write(format!("<p>HTTP/0.9: {:?}</p>", r.http09()));

   let args = unwrap_str(r.args());
   r.write(format!("<p>Request Args: {}</p>", args));

   let canonical_filename = unwrap_str(r.canonical_filename());
   r.write(format!("<p>Canonical Filename: {}</p>", canonical_filename));

   let auth_type = unwrap_str(r.auth_type());
   r.write(format!("<p>Auth Type: {}</p>", auth_type));

   let auth_name = unwrap_str(r.auth_name());
   r.write(format!("<p>Auth Name: {}</p>", auth_name));

   let basic_auth_pw = unwrap_str(r.basic_auth_pw());
   r.write(format!("<p>Basic Auth PW: {}</p>", basic_auth_pw));


   r.write("</body></html>");

   Status::OK
}
