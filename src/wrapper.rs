use std::str;
use std::ffi::CStr;
use libc::c_char;


pub trait FromRaw<T> {
    fn from_raw(T) -> Option<Self>;
}

pub struct Wrapper<'a, T: 'a> {
   pub raw: &'a mut T
}

impl<'a, T> FromRaw<*mut T> for Wrapper<'a, T> {
   fn from_raw(ptr: *mut T) -> Option<Wrapper<'a, T>> {
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

pub trait CType {
   type c_type;
}

impl<'a, T> CType for Wrapper<'a, T> {
   type c_type = T;
}

#[inline]
pub fn from_char_ptr<'a>(ptr: *const c_char) -> Option<&'a str> {
   if ptr.is_null() {
      return None;
   }

   let slice = unsafe { CStr::from_ptr(ptr) };
   str::from_utf8(slice.to_bytes()).ok()
}
