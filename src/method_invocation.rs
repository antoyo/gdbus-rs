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

//! `GDBusMethodInvocation` — Object for handling remote calls.

use gio_sys::{GDBusMethodInvocation, g_dbus_method_invocation_return_value};

use variant::ToVariant;

/// Object for handling remote calls.
pub struct MethodInvocation(*mut GDBusMethodInvocation);

impl MethodInvocation {
    /// Create a new method invocation from a pointer.
    pub fn new(invocation: *mut GDBusMethodInvocation) -> Self {
        MethodInvocation(invocation)
    }

    /// Finishes handling a D-Bus method call by returning `parameters`. If the `parameters` GVariant is floating, it is consumed.
    /// It is an error if `parameters` is not of the right format.
    /// This method will free `invocation`, you cannot use it afterwards.
    /// Since 2.48, if the method call requested for a reply not to be sent then this call will sink `parameters` and free `invocation`, but otherwise do nothing (as per the recommendations of the D-Bus specification).
    pub fn return_value<T: ToVariant>(&self, value: T) {
        unsafe { g_dbus_method_invocation_return_value(self.0, value.to_variant().to_glib()) };
    }
}
