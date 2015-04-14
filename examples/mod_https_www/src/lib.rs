#[macro_use]
extern crate apache2;

apache2_module!(https_www_handler, c_https_www_handler, https_www_module, b"mod_https_www\0");


fn https_www_handler(r: &apache2::Request) -> apache2::Status {
   r.write("Redirect to https and www.");

   apache2::Status::OK
}
