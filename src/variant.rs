/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

//! `GVariant` — strongly typed value datatype.

use std::ffi::{CStr, CString};
use std::ptr::null_mut;

use glib_sys::{GVariant, g_variant_get, g_variant_new};
use libc::c_char;

/// Strongly typed value datatype.
pub struct Variant(*mut GVariant);

impl Variant {
    /// Create a new `Variant` from a pointer.
    pub fn new(variant: *mut GVariant) -> Self {
        Variant(variant)
    }

    /// Convert the variant to the sys type.
    pub fn to_glib(&self) -> *mut GVariant {
        self.0
    }
}

/// Trait to convert a `variant` to a type.
pub trait FromVariant: Sized {
    /// Convert the `variant` to the type.
    fn from_variant(variant: &Variant) -> Self;
}

impl FromVariant for (String,) {
    fn from_variant(variant: &Variant) -> Self {
        let mut c_string: *mut c_char = null_mut();
        let format = CString::new("(&s)").unwrap();
        unsafe { g_variant_get(variant.to_glib(), format.as_ptr(), &mut c_string as *mut _) };
        let result = unsafe { CStr::from_ptr(c_string) };
        let string = result.to_str().unwrap().to_string();
        (string,)
    }
}

/// Trait to convert a type to a `Variant`.
pub trait ToVariant {
    /// Convert a type to a `Variant`.
    fn to_variant(&self) -> Variant;
}

impl<'a> ToVariant for (&'a str,) {
    fn to_variant(&self) -> Variant {
        let string = CString::new(self.0).unwrap();
        let format = CString::new("(s)").unwrap();
        Variant(unsafe { g_variant_new(format.as_ptr(), string.as_ptr()) })
    }
}

impl ToVariant for (String,) {
    fn to_variant(&self) -> Variant {
        let string = CString::new(self.0.as_bytes()).unwrap();
        let format = CString::new("(s)").unwrap();
        Variant(unsafe { g_variant_new(format.as_ptr(), string.as_ptr()) })
    }
}
