#![allow(non_camel_case_types)]

use std::ffi::CString;

use ffi;

use wrapper::{Wrapper, c_str_value};


pub enum HookOrder {
   REALLY_FIRST,  // run this hook first, before ANYTHING
   FIRST,         // run this hook first
   MIDDLE,        // run this hook somewhere
   LAST,          // run this hook after every other hook which is defined
   REALLY_LAST    // run this hook last, after EVERYTHING
}

impl Into<::libc::c_int> for HookOrder {
   fn into(self) -> ::libc::c_int {
      match self {
         HookOrder::REALLY_FIRST => ffi::APR_HOOK_REALLY_FIRST,
         HookOrder::FIRST => ffi::APR_HOOK_FIRST,
         HookOrder::MIDDLE => ffi::APR_HOOK_MIDDLE,
         HookOrder::LAST => ffi::APR_HOOK_LAST,
         HookOrder::REALLY_LAST => ffi::APR_HOOK_REALLY_LAST
      }
   }
}


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
