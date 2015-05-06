# Apache module development with Rust

This library is intended to make possible and easy writing Apache HTTP Server modules using
the Rust programming language.

*NOTE: The library is still in early development phase – extensive documentation will be provided soon after the basic features are implemented*

Here is a small example of what an Apache Rust module code looks like:

```rust
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
```
