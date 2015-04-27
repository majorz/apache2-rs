use std::str;
use std::ffi::CStr;
use libc::c_char;

use ffi::{UTF8_DECODE_ERROR, NULL_PTR_ERROR};


pub struct Wrapper<'a, T: 'a> {
   pub raw: &'a mut T
}

impl<'a, T> Wrapper<'a, T> {
   pub fn from_raw_ptr(ptr: *mut T) -> Option<Self> {
      if ptr.is_null() {
         None
      } else {
         let raw: &mut T = unsafe { &mut *ptr };
         Some(
            Wrapper::<T> {
               raw: raw
            }
         )
      }
   }

}

#[inline]
pub fn from_char_ptr<'a>(ptr: *const c_char) -> Result<&'a str, &'static str> {
   if ptr.is_null() {
      return Err(NULL_PTR_ERROR);
   }

   let slice = unsafe { CStr::from_ptr(ptr) };
   match str::from_utf8(slice.to_bytes()) {
      Ok(s) => Ok(s),
      Err(_) => Err(UTF8_DECODE_ERROR)
   }
}

#[inline]
pub fn wrap_ptr<'a, T>(ptr: *mut T) -> Option<Wrapper<'a, T>> {
   if ptr.is_null() {
      return None
   }

   Wrapper::<'a, T>::from_raw_ptr(ptr)
}
