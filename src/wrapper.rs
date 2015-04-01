use std::str;
use std::ffi::CStr;
use libc::c_char;


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
pub fn c_str_value<'a>(ptr: *const c_char) -> Option<&'a str> {
   if ptr.is_null() {
      return None
   }

   let data = unsafe { CStr::from_ptr(ptr) }.to_bytes();
   match str::from_utf8(data) {
      Ok(s) => Some(s),
      Err(_) => None
   }
}
