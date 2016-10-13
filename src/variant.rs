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
use std::mem::zeroed;

use glib_sys::{GVariant, g_variant_get, g_variant_new};
use libc::{c_char, c_int, int16_t, int32_t, int64_t, uint16_t, uint32_t, uint64_t, uint8_t};

/// Wrapper for boolean c type.
pub struct CBool(c_int);

macro_rules! numeric_variant {
    ($rust_type:ty, $c_type:ty, $format:expr) => {
        impl FromFFI for $rust_type {
            type Input = $c_type;

            unsafe fn from_ffi(input: Self::Input) -> Self {
                input
            }
        }

        impl FromFormat for $rust_type {
            fn from_format() -> &'static str {
                $format
            }
        }

        impl ToArg for $c_type {
            type Output = $c_type;

            fn to_arg(&self) -> Self::Output {
                *self
            }
        }

        impl ToFFI for $rust_type {
            type Output = $c_type;

            fn to_ffi(&self) -> Self::Output {
                *self
            }
        }

        impl ToFormat for $rust_type {
            fn to_format() -> &'static str {
                $format
            }
        }
    };
}

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

impl FromFFI for bool {
    type Input = c_int;

    unsafe fn from_ffi(input: Self::Input) -> Self {
        input != 0
    }
}

impl FromFormat for bool {
    fn from_format() -> &'static str {
        "b"
    }
}

impl ToArg for CBool {
    type Output = c_int;

    fn to_arg(&self) -> Self::Output {
        self.0
    }
}

impl ToFFI for bool {
    type Output = CBool;

    fn to_ffi(&self) -> Self::Output {
        CBool(*self as i32)
    }
}

impl ToFormat for bool {
    fn to_format() -> &'static str {
        "b"
    }
}

numeric_variant!(u8, uint8_t, "y");
numeric_variant!(i16, int16_t, "n");
numeric_variant!(u16, uint16_t, "q");
numeric_variant!(i32, int32_t, "i");
numeric_variant!(u32, uint32_t, "u");
numeric_variant!(i64, int64_t, "x");
numeric_variant!(u64, uint64_t, "t");

/// Trait for converting to a value from its ffi representation.
pub trait FromFFI {
    /// Rust representation.
    type Input;

    /// Convert the value from its ffi representation.
    unsafe fn from_ffi(input: Self::Input) -> Self;
}

impl FromFFI for char {
    type Input = c_char;

    unsafe fn from_ffi(input: Self::Input) -> Self {
        input as u8 as Self
    }
}

impl<'a> FromFFI for &'a str {
    type Input = *mut c_char;

    unsafe fn from_ffi(input: Self::Input) -> Self {
        let result = CStr::from_ptr(input);
        result.to_str().unwrap()
    }
}

impl FromFFI for String {
    type Input = *mut c_char;

    unsafe fn from_ffi(input: Self::Input) -> Self {
        let result = CStr::from_ptr(input);
        result.to_str().unwrap().to_string()
    }
}

/// Trait for converting a type to its format string for a conversion from a variant.
pub trait FromFormat {
    /// Convert the type to its from format string.
    fn from_format() -> &'static str;
}

impl FromFormat for char {
    fn from_format() -> &'static str {
        "y"
    }
}

impl<'a> FromFormat for &'a str {
    fn from_format() -> &'static str {
        "&s"
    }
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
        let mut ffi: <P as FromFFI>::Input = unsafe { zeroed() };
        let format = CString::new(format!("({})", P::from_format()).as_bytes()).unwrap();
        unsafe { g_variant_get(variant.to_glib(), format.as_ptr(), &mut ffi as *mut _) };
        (unsafe { P::from_ffi(ffi) },)
    }
}

impl<P: FromFFI + FromFormat, Q: FromFFI + FromFormat> FromVariant for (P, Q) {
    fn from_variant(variant: &Variant) -> Self {
        let mut ffi1: <P as FromFFI>::Input = unsafe { zeroed() };
        let mut ffi2: <Q as FromFFI>::Input = unsafe { zeroed() };
        let format = CString::new(format!("({}{})", P::from_format(), Q::from_format()).as_bytes()).unwrap();
        unsafe { g_variant_get(variant.to_glib(), format.as_ptr(), &mut ffi1 as *mut _, &mut ffi2 as *mut _) };
        unsafe { (P::from_ffi(ffi1), Q::from_ffi(ffi2)) }
    }
}

impl<P: FromFFI + FromFormat, Q: FromFFI + FromFormat, R: FromFFI + FromFormat> FromVariant for (P, Q, R) {
    fn from_variant(variant: &Variant) -> Self {
        let mut ffi1: <P as FromFFI>::Input = unsafe { zeroed() };
        let mut ffi2: <Q as FromFFI>::Input = unsafe { zeroed() };
        let mut ffi3: <R as FromFFI>::Input = unsafe { zeroed() };
        let format = CString::new(format!("({}{}{})",
            P::from_format(),
            Q::from_format(),
            R::from_format()
        ).as_bytes()).unwrap();
        unsafe { g_variant_get(variant.to_glib(), format.as_ptr(),
            &mut ffi1 as *mut _,
            &mut ffi2 as *mut _,
            &mut ffi3 as *mut _,
        )};
        unsafe { (
            P::from_ffi(ffi1),
            Q::from_ffi(ffi2),
            R::from_ffi(ffi3),
        )}
    }
}

impl<P: FromFFI + FromFormat, Q: FromFFI + FromFormat, R: FromFFI + FromFormat, S: FromFFI + FromFormat> FromVariant for (P, Q, R, S) {
    fn from_variant(variant: &Variant) -> Self {
        let mut ffi1: <P as FromFFI>::Input = unsafe { zeroed() };
        let mut ffi2: <Q as FromFFI>::Input = unsafe { zeroed() };
        let mut ffi3: <R as FromFFI>::Input = unsafe { zeroed() };
        let mut ffi4: <S as FromFFI>::Input = unsafe { zeroed() };
        let format = CString::new(format!("({}{}{}{})",
            P::from_format(),
            Q::from_format(),
            R::from_format(),
            S::from_format(),
        ).as_bytes()).unwrap();
        unsafe { g_variant_get(variant.to_glib(), format.as_ptr(),
            &mut ffi1 as *mut _,
            &mut ffi2 as *mut _,
            &mut ffi3 as *mut _,
            &mut ffi4 as *mut _,
        )};
        unsafe { (
            P::from_ffi(ffi1),
            Q::from_ffi(ffi2),
            R::from_ffi(ffi3),
            S::from_ffi(ffi4),
        )}
    }
}

impl<P: FromFFI + FromFormat, Q: FromFFI + FromFormat, R: FromFFI + FromFormat, S: FromFFI + FromFormat, T: FromFFI + FromFormat> FromVariant for (P, Q, R, S, T) {
    fn from_variant(variant: &Variant) -> Self {
        let mut ffi1: <P as FromFFI>::Input = unsafe { zeroed() };
        let mut ffi2: <Q as FromFFI>::Input = unsafe { zeroed() };
        let mut ffi3: <R as FromFFI>::Input = unsafe { zeroed() };
        let mut ffi4: <S as FromFFI>::Input = unsafe { zeroed() };
        let mut ffi5: <T as FromFFI>::Input = unsafe { zeroed() };
        let format = CString::new(format!("({}{}{}{}{})",
            P::from_format(),
            Q::from_format(),
            R::from_format(),
            S::from_format(),
            T::from_format(),
        ).as_bytes()).unwrap();
        unsafe { g_variant_get(variant.to_glib(), format.as_ptr(),
            &mut ffi1 as *mut _,
            &mut ffi2 as *mut _,
            &mut ffi3 as *mut _,
            &mut ffi4 as *mut _,
            &mut ffi5 as *mut _,
        )};
        unsafe { (
            P::from_ffi(ffi1),
            Q::from_ffi(ffi2),
            R::from_ffi(ffi3),
            S::from_ffi(ffi4),
            T::from_ffi(ffi5),
        )}
    }
}

/// Trait for converting a ffi value to an argument for `g_variant_new()`.
pub trait ToArg {
    /// Representation required for `g_variant_new()`.
    type Output;

    /// Convert the ffi value to an argument.
    fn to_arg(&self) -> Self::Output;
}

impl ToArg for c_char {
    type Output = c_char;

    fn to_arg(&self) -> Self::Output {
        *self
    }
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

impl ToFFI for char {
    type Output = c_char;

    fn to_ffi(&self) -> Self::Output {
        *self as c_char
    }
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

impl ToFormat for char {
    fn to_format() -> &'static str {
        "y"
    }
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

impl<P: ToFFI + ToFormat, Q: ToFFI + ToFormat> ToVariant for (P, Q) {
    fn to_variant(&self) -> Variant {
        let ffi1 = self.0.to_ffi();
        let ffi2 = self.1.to_ffi();
        let format = CString::new(format!("({}{})", P::to_format(), Q::to_format()).as_bytes()).unwrap();
        Variant(unsafe { g_variant_new(format.as_ptr(), ffi1.to_arg(), ffi2.to_arg()) })
    }
}

impl<P: ToFFI + ToFormat, Q: ToFFI + ToFormat, R: ToFFI + ToFormat> ToVariant for (P, Q, R) {
    fn to_variant(&self) -> Variant {
        let ffi1 = self.0.to_ffi();
        let ffi2 = self.1.to_ffi();
        let ffi3 = self.2.to_ffi();
        let format = CString::new(format!("({}{}{})", P::to_format(), Q::to_format(), R::to_format()).as_bytes()).unwrap();
        Variant(unsafe { g_variant_new(format.as_ptr(), ffi1.to_arg(), ffi2.to_arg(), ffi3.to_arg()) })
    }
}

impl<P: ToFFI + ToFormat, Q: ToFFI + ToFormat, R: ToFFI + ToFormat, S: ToFFI + ToFormat> ToVariant for (P, Q, R, S) {
    fn to_variant(&self) -> Variant {
        let ffi1 = self.0.to_ffi();
        let ffi2 = self.1.to_ffi();
        let ffi3 = self.2.to_ffi();
        let ffi4 = self.3.to_ffi();
        let format = CString::new(format!("({}{}{}{})", P::to_format(), Q::to_format(), R::to_format(), S::to_format()).as_bytes()).unwrap();
        Variant(unsafe { g_variant_new(format.as_ptr(), ffi1.to_arg(), ffi2.to_arg(), ffi3.to_arg(), ffi4.to_arg()) })
    }
}

impl<P: ToFFI + ToFormat, Q: ToFFI + ToFormat, R: ToFFI + ToFormat, S: ToFFI + ToFormat, T: ToFFI + ToFormat> ToVariant for (P, Q, R, S, T) {
    fn to_variant(&self) -> Variant {
        let ffi1 = self.0.to_ffi();
        let ffi2 = self.1.to_ffi();
        let ffi3 = self.2.to_ffi();
        let ffi4 = self.3.to_ffi();
        let ffi5 = self.4.to_ffi();
        let format = CString::new(format!("({}{}{}{}{})", P::to_format(), Q::to_format(), R::to_format(), S::to_format(), T::to_format()).as_bytes()).unwrap();
        Variant(unsafe { g_variant_new(format.as_ptr(), ffi1.to_arg(), ffi2.to_arg(), ffi3.to_arg(), ffi4.to_arg(), ffi5.to_arg()) })
    }
}
