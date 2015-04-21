#[macro_use]
extern crate apache2;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::thread::sleep_ms;

use apache2::{Request, Status, HookOrder};

apache2_module!(redis_log_handler, c_redis_log_handler, redis_log_module, b"mod_redis_log\0",
   ap_hook_log_transaction, HookOrder::MIDDLE);


fn redis_log_handler(r: &mut Request) -> Status {
   sleep_ms(30000);

   let path = Path::new("/tmp/lorem_ipsum.txt");

   let mut file = File::create(&path).unwrap();

   let client_ip = r.connection().unwrap().client_ip().unwrap();

   file.write_all(client_ip.as_bytes()).unwrap();

   Status::OK
}
