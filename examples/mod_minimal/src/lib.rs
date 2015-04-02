#![feature(convert)]
#![feature(libc)]

extern crate libc;

#[macro_use]
extern crate apache2;


apache2_module!(minimal_module, minimal_handler, b"mod_minimal\0");


fn minimal_handler(r: &apache2::Request) -> apache2::Status {
   r.write("Hello, Rust!");

   apache2::Status::OK
}
