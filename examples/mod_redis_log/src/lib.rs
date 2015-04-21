#[macro_use]
extern crate apache2;
extern crate redis;

use redis::Commands;

use apache2::{Request, Status, HookOrder};

apache2_module!(redis_log_handler, c_redis_log_handler, redis_log_module, b"mod_redis_log\0",
   ap_hook_log_transaction, HookOrder::MIDDLE);


fn redis_log_handler(r: &mut Request) -> Status {
   let client = redis::Client::open("redis://127.0.0.1/").unwrap();

   let con = client.get_connection().unwrap();

   let client_ip = r.connection().unwrap().client_ip().unwrap();

   let _ : () = con.set("last_ip", client_ip).unwrap();

   Status::OK
}
