#![feature(collections)]

#[macro_use]
extern crate apache2;

apache2_module!(https_www_handler, c_https_www_handler, https_www_module, b"mod_https_www\0", ap_hook_translate_name, apache2::HookOrder::MIDDLE);


fn https_www_handler(r: &mut apache2::Request) -> apache2::Status {
   let scheme = match r.http_scheme() {
      None => {
         return apache2::Status::HTTP_INTERNAL_SERVER_ERROR;
      },
      Some(scheme) => scheme
   };

   let hostname = match r.hostname() {
      None => {
         return apache2::Status::HTTP_INTERNAL_SERVER_ERROR;
      },
      Some(hostname) => hostname
   };

   let already_www = &hostname[..4] == "www.";
   let already_https = scheme == "https";

   if already_www && already_https {
      return apache2::Status::DECLINED;
   };

   let full_hostname = if already_www {
      String::from_str(hostname)
   } else {
      format!("www.{}", hostname)
   };

   let uri = match r.unparsed_uri() {
      None => {
         return apache2::Status::HTTP_INTERNAL_SERVER_ERROR;
      },
      Some(uri) => uri
   };

   let location = format!("https://{}{}", full_hostname, uri);
   r.headers_out().unwrap().set("Location", location);

   r.set_status(apache2::Status::HTTP_MOVED_PERMANENTLY);

   apache2::Status::DONE
}
