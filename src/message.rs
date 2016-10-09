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

//! `GDBusMessage` — D-Bus Message.

use std::ffi::CString;

use gdbus_sys::{GDBusMessage, g_dbus_message_get_body, g_dbus_message_new_method_call, g_dbus_message_set_body};

use variant::{ToVariant, Variant};

/// A type for representing D-Bus messages that can be sent or received on a `GDBusConnection`.
pub struct Message(*mut GDBusMessage);

impl Message {
    /// Create a new message from a pointer.
    pub fn new(message: *mut GDBusMessage) -> Self {
        Message(message)
    }

    /// Creates a new `GDBusMessage` for a method call.
    pub fn new_method_call(name: &str, path: &str, interface: &str, method: &str) -> Self {
        let name = CString::new(name).unwrap();
        let path = CString::new(path).unwrap();
        let interface = CString::new(interface).unwrap();
        let method = CString::new(method).unwrap();
        let message = unsafe { g_dbus_message_new_method_call(name.as_ptr(), path.as_ptr(), interface.as_ptr(), method.as_ptr()) };
        Message(message)
    }

    /// Gets the body of a message.
    pub fn get_body(&self) -> Variant {
        Variant::new(unsafe { g_dbus_message_get_body(self.0) })
    }

    /// Sets the body `message`. As a side-effect the `G_DBUS_MESSAGE_HEADER_FIELD_SIGNATURE` header field is set to the type string of `body` (or cleared if `body` is `NULL`).
    /// If `body` is floating, `message` assumes ownership of `body`.
    pub fn set_body<T: ToVariant>(&self, variant: T) {
        unsafe { g_dbus_message_set_body(self.0, variant.to_variant().to_glib()) };
    }

    /// Convert to the sys type.
    pub fn to_glib(&self) -> *mut GDBusMessage {
        self.0
    }
}
