#[macro_use]
extern crate apache2;

use apache2::{Request, Status, StatusResult, server_banner, server_description, server_built,
   show_mpm, apr_version_string, apu_version_string, Cookie, time_now};

apache2_module!(info_rs_handler, c_info_rs_handler, info_rs_module, b"mod_info_rs\0");

fn unwrap_str<'a>(option: Result<&'a str, &'static str>) -> &'a str {
   match option {
      Ok(val) => val,
      Err(_) => "--"
   }
}

fn info_rs_handler(r: &mut Request) -> StatusResult {
   if try!(r.handler()) != "server-info-rs" {
      return Ok(Status::DECLINED)
   }

   r.set_content_type("text/html");

   r.set_last_modified(time_now());

   try!(r.write("<!doctype html><html><head><meta charset=\"utf-8\"><title>Apache Info</title></head><body>"));

   try!(r.write("<h1>Apache Server Information</h1>"));

   let server_name = unwrap_str(
      r.escape_html(
         unwrap_str(r.server_name())
      )
   );
   let server_port = r.server_port();
   let local_ip = unwrap_str(try!(r.connection()).local_ip());
   try!(r.write(format!("<p>Server: {}:{} (via {})</p>", server_name, server_port, local_ip)));

   let description = unwrap_str(server_description());
   let banner = unwrap_str(server_banner());
   try!(r.write(format!("<p>Server Description/Banner: {} / {}</p>", description, banner)));

   let mmp = unwrap_str(show_mpm());
   try!(r.write(format!("<p>Server MPM: {}</p>", mmp)));

   let built = unwrap_str(server_built());
   try!(r.write(format!("<p>Server Built: {}</p>", built)));

   let apr_version = unwrap_str(apr_version_string());
   try!(r.write(format!("<p>Server loaded APR Version: {}</p>", apr_version)));

   let apu_version = unwrap_str(apu_version_string());
   try!(r.write(format!("<p>Server loaded APU Version: {}</p>", apu_version)));

   let document_root = unwrap_str(r.document_root());
   try!(r.write(format!("<p>Document Root: {}</p>", document_root)));

   try!(r.write("<hr />"));

   try!(r.write("<h2>Current Request Information</h2>"));

   let client_ip = unwrap_str(try!(r.connection()).client_ip());
   try!(r.write(format!("<p>Client IP: {}</p>", client_ip)));

   let useragent_ip = unwrap_str(r.useragent_ip());
   try!(r.write(format!("<p>Useragent IP: {}</p>", useragent_ip)));

   let hostname = unwrap_str(r.hostname());
   try!(r.write(format!("<p>Hostname: {}</p>", hostname)));

   let the_request = unwrap_str(r.the_request());
   try!(r.write(format!("<p>Request: {}</p>", the_request)));

   let protocol = unwrap_str(r.protocol());
   try!(r.write(format!("<p>Protocol: {}</p>", protocol)));

   let http_scheme = unwrap_str(r.http_scheme());
   try!(r.write(format!("<p>HTTP Scheme: {}</p>", http_scheme)));

   try!(r.write(format!("<p>HTTP/0.9: {:?}</p>", r.http09())));

   let method = unwrap_str(r.method());
   try!(r.write(format!("<p>Method: {}</p>", method)));

   let unparsed_uri = unwrap_str(r.unparsed_uri());
   try!(r.write(format!("<p>Unparsed URI: {}</p>", unparsed_uri)));

   let uri = unwrap_str(r.uri());
   try!(r.write(format!("<p>URI: {}</p>", uri)));

   let args = unwrap_str(r.args());
   try!(r.write(format!("<p>Request Args: {}</p>", args)));

   let content_type = unwrap_str(r.content_type());
   try!(r.write(format!("<p>Content Type: {}</p>", content_type)));

   let content_encoding = unwrap_str(r.content_encoding());
   try!(r.write(format!("<p>Content Encoding: {}</p>", content_encoding)));

   try!(r.write(format!("<p>Content Length: {}</p>", r.clength())));

   try!(r.write(format!("<p>Is Initial Request: {}</p>", r.is_initial_req())));

   let context_document_root = unwrap_str(r.context_document_root());
   try!(r.write(format!("<p>Context Document Root: {}</p>", context_document_root)));

   let context_prefix = unwrap_str(r.context_prefix());
   try!(r.write(format!("<p>Context Prefix: {}</p>", context_prefix)));

   let range = unwrap_str(r.range());
   try!(r.write(format!("<p>Range: {}</p>", range)));

   let handler = unwrap_str(r.handler());
   try!(r.write(format!("<p>Handler: {}</p>", handler)));

   let path_info = unwrap_str(r.path_info());
   try!(r.write(format!("<p>Path Info: {}</p>", path_info)));

   let filename = unwrap_str(r.filename());
   try!(r.write(format!("<p>Filename: {}</p>", filename)));

   let canonical_filename = unwrap_str(r.canonical_filename());
   try!(r.write(format!("<p>Canonical Filename: {}</p>", canonical_filename)));

   let request_time = unwrap_str(r.rfc822_date(r.request_time()));
   try!(r.write(format!("<p>Request Time: {} / {}</p>", request_time, r.request_time())));

   let mtime = unwrap_str(r.rfc822_date(r.mtime()));
   try!(r.write(format!("<p>Last modified time: {} / {}</p>", mtime, r.mtime())));

   let log_id = unwrap_str(r.log_id());
   try!(r.write(format!("<p>Log ID: {}</p>", log_id)));

   let user = unwrap_str(r.user());
   try!(r.write(format!("<p>User: {}</p>", user)));

   try!(r.write(format!("<p>Some Auth Required: {}</p>", r.some_auth_required())));

   let auth_type = unwrap_str(r.auth_type());
   try!(r.write(format!("<p>Auth Type: {}</p>", auth_type)));

   let auth_name = unwrap_str(r.auth_name());
   try!(r.write(format!("<p>Auth Name: {}</p>", auth_name)));

   let basic_auth_pw = unwrap_str(r.basic_auth_pw());
   try!(r.write(format!("<p>Basic Auth PW: {}</p>", basic_auth_pw)));

   try!(r.write(format!("<p>Default Port: {}</p>", r.default_port())));

   try!(r.write(format!("<p>ProxyReq: {}</p>", r.proxyreq())));

   let key = "sample_cookie";
   let val = "info_rs";
   match r.cookie(key) {
      Err(_) => {
         let mut cookie = Cookie::new(key, val);
         cookie.expires = Some(time_now() + 1000000 * 30);

         r.set_cookie(cookie);
         try!(r.write(format!("<p>New Cookie – {}: {}</p>", key, val)));
      },
      Ok(stored) => {
         try!(r.write(format!("<p>Cookie – {}: {}</p>", key, stored)));
      }
   };

   try!(r.write("<h3>Request Headers</h3>"));

   let headers_in = try!(r.headers_in());

   for (key, val) in headers_in.iter() {
      try!(r.write(format!("<p>{}: {}</p>", key, unwrap_str(val))));
   }

   try!(r.write("<h3>Headers Out</h3>"));

   let headers_out = try!(r.headers_out());

   for (key, val) in headers_out.iter() {
      try!(r.write(format!("<p>{}: {}</p>", key, unwrap_str(val))));
   }

   try!(r.write("<h3>Err Headers Out</h3>"));

   let err_headers_out = try!(r.err_headers_out());

   for (key, val) in err_headers_out.iter() {
      try!(r.write(format!("<p>{}: {}</p>", key, unwrap_str(val))));
   }

   try!(r.write("<h3>Notes</h3>"));

   let notes = try!(r.notes());

   for (key, val) in notes.iter() {
      try!(r.write(format!("<p>{}: {}</p>", key, unwrap_str(val))));
   }

   try!(r.write("<h3>Subprocess Environment</h3>"));

   let subprocess_env = try!(r.subprocess_env());

   for (key, val) in subprocess_env.iter() {
      try!(r.write(format!("<p>{}: {}</p>", key, unwrap_str(val))));
   }

   try!(r.write("<h3>Request API check</h3>"));

   let original = "Բարեւ, Héébee, გამარჯობა, Witôjze, Здраво, Ciao";
   let encoded = try!(r.base64_encode(original));
   let plain = try!(r.base64_decode(encoded));
   try!(r.write(format!("<p>Original Text: {}</p>", original)));
   try!(r.write(format!("<p>Base64 Encoded: {}</p>", encoded)));
   try!(r.write(format!("<p>Base64 Decoded: {}</p>", plain)));

   let original_url = "http://foo.bar/1 2 3 & 4 + 5";
   let encoded_url = try!(r.escape_urlencoded(original_url));
   let plain_url = try!(r.unescape_urlencoded(encoded_url));
   try!(r.write(format!("<p>Original URL: {}</p>", original_url)));
   try!(r.write(format!("<p>Encoded URL: {}</p>", encoded_url)));
   try!(r.write(format!("<p>Decoded URL: {}</p>", plain_url)));

   let date = unwrap_str(r.rfc822_date(0));
   try!(r.write(format!("<p>RFC 822 Date: {}</p>", date)));

   try!(r.write("</body></html>"));

   Ok(Status::OK)
}
