#![feature(convert)]
#![feature(libc)]

extern crate libc;

#[macro_use] extern crate apache2;

use apache2::{Request, Status};


apache2_module!(hellorust_module, hellorust_handler, b"mod_hellorust\0");


fn hellorust_handler(r: &Request) -> Status {
   r.write("Hello, Rust!");

   Status::OK
}
