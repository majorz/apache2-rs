use time;


#[derive(PartialEq, Clone, Debug)]
pub struct Cookie {
   pub name: String,
   pub value: String,
   pub expires: Option<time::Tm>,
   pub max_age: Option<u64>,
   pub domain: Option<String>,
   pub path: Option<String>,
   pub secure: bool,
   pub httponly: bool,
}

impl Cookie {
   pub fn new(name: String, value: String) -> Cookie {
      Cookie {
         name: name,
         value: value,
         expires: None,
         max_age: None,
         domain: None,
         path: Some("/".to_string()),
         secure: false,
         httponly: false,
      }
   }

   pub fn attrs(&self) -> String {
      let mut res = String::new();

      if self.httponly {
         res.push_str(";HttpOnly");
      }

      if self.secure {
         res.push_str(";Secure");
      }

      match self.path {
         Some(ref s) => res.push_str(format!(";Path={}", s).as_ref()),
         None => {}
      }

      match self.domain {
         Some(ref s) => res.push_str(format!(";Domain={}", s).as_ref()),
         None => {}
      }

      match self.max_age {
         Some(n) => res.push_str(format!(";Max-Age={}", n).as_ref()),
         None => {}
      }

      match self.expires {
         Some(ref t) => res.push_str(format!(";Expires={}", t.rfc822()).as_ref()),
         None => {}
      }

      if res.len() > 0 {
         res.remove(0);
      }

      res
   }
}
