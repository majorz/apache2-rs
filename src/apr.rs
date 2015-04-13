use std::ffi::CString;

use ffi;

use wrapper::{Wrapper, c_str_value};


pub type AprTable<'a> = Wrapper<'a, ffi::apr_table_t>;


impl<'a> AprTable<'a> {
   pub fn get(&self, key: &'static str) -> Option<&'a str> {
      c_str_value(
         unsafe { ffi::apr_table_get(self.raw, CString::new(key).unwrap().as_ptr()) }
      )
   }

   pub fn set(&mut self, key: &'static str, val: &'static str) {
      unsafe {
         ffi::apr_table_set(
            self.raw,
            CString::new(key).unwrap().as_ptr(),
            CString::new(val).unwrap().as_ptr()
         )
      };
   }

   pub fn iter(&self) -> AprTableIter {
      let ptr = unsafe { ffi::apr_table_elts(self.raw) };
      let raw: &ffi::apr_array_header_t = unsafe { &*ptr };

      AprTableIter {
         array_header: raw,
         next_idx: 0
      }
   }
}

pub struct AprTableIter<'a> {
   array_header: &'a ffi::apr_array_header_t,
   next_idx: usize,
}

impl<'a> Iterator for AprTableIter<'a> {
   type Item = (&'a str, &'a str);

   fn next(&mut self) -> Option<(&'a str, &'a str)> {
      if self.next_idx != self.array_header.nelts as usize {
         let mut elts = self.array_header.elts as *const ffi::apr_table_entry_t;

         elts = unsafe { elts.offset(self.next_idx as isize) };
         self.next_idx += 1;

         let key = c_str_value(unsafe { (*elts).key }).unwrap();
         let val = c_str_value(unsafe { (*elts).val }).unwrap();

         Some((key, val))
      } else {
         None
      }
   }

   fn size_hint(&self) -> (usize, Option<usize>) {
      let rem = self.array_header.nelts as usize - self.next_idx;
      (rem, Some(rem))
   }
}

pub fn apr_version_string<'a>() -> Option<&'a str> {
   c_str_value(
      unsafe { ffi::apr_version_string() }
   )
}

pub fn apu_version_string<'a>() -> Option<&'a str> {
   c_str_value(
      unsafe { ffi::apu_version_string() }
   )
}
