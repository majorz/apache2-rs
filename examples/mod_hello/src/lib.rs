#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use]
extern crate apache2;

use apache2::{Request, Status};

apache2_module!(hello, b"mod_hello\0");


fn hello_handler(r: &mut Request) -> Result<Status, ()> {
   r.set_content_type("text/plain; charset=utf-8");

   try!(r.write("Hello Ciao Здравейте"));

   Ok(Status::OK)
}
