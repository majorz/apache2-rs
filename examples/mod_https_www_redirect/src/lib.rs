extern crate libc;

#[macro_use]
extern crate apache2;

apache2_module!(https_www_redirect_handler, c_https_www_redirect_handler, https_www_redirect_module, b"mod_https_www_redirect\0");


fn https_www_redirect_handler(r: &apache2::Request) -> apache2::Status {
   r.write("https www redirect module");

   apache2::Status::OK
}
