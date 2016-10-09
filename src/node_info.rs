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

//! D-Bus Introspection Data — Node and interface description data structures.

use std::ffi::CString;
use std::ptr::null_mut;

use gdbus_sys::{GDBusInterfaceInfo, GDBusNodeInfo, g_dbus_node_info_new_for_xml, g_dbus_node_info_unref};
use glib::Error;
use glib::translate::from_glib_full;

/// Information about a D-Bus interface.
pub struct InterfaceInfo(*mut GDBusInterfaceInfo);

impl InterfaceInfo {
    /// Convert to the sys type.
    pub fn to_glib(&self) -> *mut GDBusInterfaceInfo {
        self.0
    }
}

/// Information about nodes in a remote object hierarchy.
pub struct NodeInfo(*mut GDBusNodeInfo);

impl NodeInfo {
    /// Create a `NodeInfo` from a pointer.
    pub fn new(node_info: *mut GDBusNodeInfo) -> Self {
        NodeInfo(node_info)
    }

    /// Parses `xml_data` and returns a `NodeInfo` representing the data.
    /// The introspection XML must contain exactly one top-level <node> element.
    /// Note that this routine is using a GMarkup-based parser that only accepts a subset of valid XML documents.
    pub fn new_for_xml(xml_data: &str) -> Result<Self, Error> {
        let mut error = null_mut();
        let xml_data = CString::new(xml_data).unwrap();
        let node_info = unsafe { g_dbus_node_info_new_for_xml(xml_data.as_ptr(), &mut error) };
        if error.is_null() {
            Ok(NodeInfo(node_info))
        }
        else {
            Err(unsafe { from_glib_full(error) })
        }
    }

    /// Return an interface from the node by its `index`.
    pub fn interface(&self, index: isize) -> InterfaceInfo {
        InterfaceInfo(unsafe { *(*self.0).interfaces.offset(index) })
    }
}

impl Drop for NodeInfo {
    fn drop(&mut self) {
        unsafe { g_dbus_node_info_unref(self.0) };
    }
}
