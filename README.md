# Apache module development with Rust

This library is intended to make possible and easy writing Apache HTTP Server modules using
the Rust programming language.

*The library is still in early development phase*

Here is a small example of what an Apache Rust module code looks like:

```rust
#[macro_use]
extern crate apache2;

use apache2::{Request, Status};

apache2_module!(hello_handler, c_hello_handler, hello_module, b"mod_hello\0");


fn hello_handler(r: &mut Request) -> Result<Status, ()> {
   r.set_content_type("text/plain; charset=utf-8");

   try!(r.write("Hello Ciao Здравейте Γεια σας مرحبا Բարեւ ສະບາຍດີ Ձեզ Բարեւ გამარჯობა"));

   Ok(Status::OK)
}
```
