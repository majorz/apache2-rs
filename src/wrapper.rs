#![allow(raw_pointer_derive)]

use std::str;
use std::ffi::CStr;
use libc::c_char;


pub trait FromRaw<T> {
   fn from_raw(T) -> Option<Self>;
}

#[derive(Copy, Clone)]
pub struct Wrapper<T: Copy + Clone> {
   pub ptr: *mut T
}

impl<T: Copy + Clone> FromRaw<*mut T> for Wrapper<T> {
   fn from_raw(ptr: *mut T) -> Option<Wrapper<T>> {
      if ptr.is_null() {
         None
      } else {
         Some(
            Wrapper::<T> {
               ptr: ptr
            }
         )
      }
   }
}

pub trait WrappedType {
   type wrapped_type;
}

impl<T: Copy + Clone> WrappedType for Wrapper<T> {
   type wrapped_type = T;
}

#[inline]
pub fn from_char_ptr<'a>(ptr: *const c_char) -> Option<&'a str> {
   if ptr.is_null() {
      return None;
   }

   let slice = unsafe { CStr::from_ptr(ptr) };
   str::from_utf8(slice.to_bytes()).ok()
}
