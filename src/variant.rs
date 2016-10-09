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

/// Trait for converting to a value from its ffi representation.
pub trait FromFFI {
    /// Rust representation.
    type Input;

    /// Convert the value from its ffi representation.
    unsafe fn from_ffi(input: *mut Self::Input) -> Self;
}

impl FromFFI for String {
    type Input = c_char;

    unsafe fn from_ffi(input: *mut Self::Input) -> Self {
        let result = CStr::from_ptr(input);
        result.to_str().unwrap().to_string()
    }
}

/// Trait for converting a type to its format string for a conversion from a variant.
pub trait FromFormat {
    /// Convert the type to its from format string.
    fn from_format() -> &'static str;
}

impl FromFormat for String {
    fn from_format() -> &'static str {
        "&s"
    }
}

/// Trait to convert a `variant` to a type.
pub trait FromVariant: Sized {
    /// Convert the `variant` to the type.
    fn from_variant(variant: &Variant) -> Self;
}

impl<P: FromFFI + FromFormat> FromVariant for (P,) {
    fn from_variant(variant: &Variant) -> Self {
        let mut ffi: *mut <P as FromFFI>::Input = null_mut();
        let format = CString::new(format!("({})", P::from_format()).as_bytes()).unwrap();
        unsafe { g_variant_get(variant.to_glib(), format.as_ptr(), &mut ffi as *mut _) };
        (unsafe { P::from_ffi(ffi) },)
    }
}

/// Trait for converting a ffi value to an argument for `g_variant_new()`.
pub trait ToArg {
    /// Representation required for `g_variant_new()`.
    type Output;

    /// Convert the ffi value to an argument.
    fn to_arg(&self) -> Self::Output;
}

impl ToArg for CString {
    type Output = *const c_char;

    fn to_arg(&self) -> Self::Output {
        self.as_ptr()
    }
}

/// Trait for converting a value to its ffi representation.
pub trait ToFFI
    where Self::Output: ToArg
{
    /// FFI representation.
    type Output;

    /// Convert the value to its ffi representation.
    fn to_ffi(&self) -> Self::Output;
}

impl<'a> ToFFI for &'a str {
    type Output = CString;

    fn to_ffi(&self) -> Self::Output {
        CString::new(*self).unwrap()
    }
}

impl ToFFI for String {
    type Output = CString;

    fn to_ffi(&self) -> Self::Output {
        CString::new(self.as_bytes()).unwrap()
    }
}

/// Trait for converting a type to its format string for a conversion to a variant.
pub trait ToFormat {
    /// Convert the type to its format string.
    fn to_format() -> &'static str;
}

impl<'a> ToFormat for &'a str {
    fn to_format() -> &'static str {
        "s"
    }
}

impl ToFormat for String {
    fn to_format() -> &'static str {
        "s"
    }
}

/// Trait to convert a type to a `Variant`.
pub trait ToVariant {
    /// Convert a type to a `Variant`.
    fn to_variant(&self) -> Variant;
}

impl<P: ToFFI + ToFormat> ToVariant for (P,) {
    fn to_variant(&self) -> Variant {
        let ffi = self.0.to_ffi();
        let format = CString::new(format!("({})", P::to_format()).as_bytes()).unwrap();
        Variant(unsafe { g_variant_new(format.as_ptr(), ffi.to_arg()) })
    }
}
