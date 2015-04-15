#[macro_use]
extern crate apache2;

apache2_module!(hello_handler);

fn hello_handler(r: &mut apache2::Request) -> apache2::Status {
   r.set_content_type("text/plain; charset=utf-8");

   r.write("Hello Ciao Здравейте Γεια σας مرحبا Բարեւ ສະບາຍດີ Ձեզ Բարեւ გამარჯობა");

   apache2::Status::OK
}
