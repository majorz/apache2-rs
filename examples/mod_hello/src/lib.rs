extern crate libc;

#[macro_use]
extern crate apache2;

apache2_module!(hello_handler, c_hello_handler, hello_module, b"mod_hello\0");


fn hello_handler(r: &apache2::Request) -> apache2::Status {
   r.set_content_type("text/html");

   r.write(
      "<!doctype html><html><head><meta charset=\"utf-8\"></head><body>
         Hello Haló Ciao Здравейте Γεια σας مرحبا Բարեւ ສະບາຍດີ Ձեզ Héébee Բարեւ გამარჯობა
      </body></html>"
   );

   apache2::Status::OK
}
