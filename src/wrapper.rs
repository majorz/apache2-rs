use std::str;
use std::ffi::CStr;
use libc::c_char;


pub struct Wrapper<'a, T: 'a> {
   pub raw: &'a mut T
}

impl<'a, T> Wrapper<'a, T> {
   pub fn from_raw_ptr(ptr: *mut T) -> Result<Self, ()> {
      if ptr.is_null() {
         Err(())
      } else {
         let raw: &mut T = unsafe { &mut *ptr };
         Ok(
            Wrapper::<T> {
               raw: raw
            }
         )
      }
   }

}

#[inline]
pub fn from_char_ptr<'a>(ptr: *const c_char) -> Result<&'a str, ()> {
   if ptr.is_null() {
      return Err(());
   }

   let slice = unsafe { CStr::from_ptr(ptr) };
   match str::from_utf8(slice.to_bytes()) {
      Ok(s) => Ok(s),
      Err(_) => Err(())
   }
}
