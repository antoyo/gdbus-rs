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

//! Bus Connections

use std::ffi::{CStr, CString};
use std::ptr::null_mut;

use gdbus_sys::{GDBusConnection, GDBusInterfaceVTable, GDBusMessage, GDBusMethodInvocation, GDBusSendMessageFlags, g_dbus_connection_register_object, g_dbus_connection_send_message_with_reply, g_dbus_connection_send_message_with_reply_finish, g_dbus_connection_send_message_with_reply_sync, g_dbus_message_get_message_type, g_dbus_message_to_gerror};
use gdbus_sys::GDBusMessageType::G_DBUS_MESSAGE_TYPE_ERROR;
use gio_sys::GAsyncResult;
use glib::error::Error;
use glib::translate::from_glib_full;
use glib_sys::{GError, GVariant};
use gobject_sys::GObject;
use libc::{c_char, c_void};

use message::Message;
use method_invocation::MethodInvocation;
use node_info::InterfaceInfo;
use own_name::user_data_free_func;
use variant::Variant;

bitflags! {
    /// Flags used in `OwnName::new()`.
    pub flags SendMessageFlags: u32 {
        /// No flags set.
        const SEND_MESSAGE_FLAGS_NONE = 0,
        /// Do not automatically assign a serial number from the `GDBusConnection` object when sending a message.
        const SEND_MESSAGE_FLAGS_PRESERVE_SERIAL = 1,
    }
}

type SendMessageCallback = Box<Box<Fn(Result<Message, Error>) + 'static>>;

/// The `GDBusConnection` type is used for D-Bus connections to remote peers such as a message buses. It is a low-level API that offers a lot of flexibility. For instance, it lets you establish a connection over any transport that can by represented as an `GIOStream`.
pub struct Connection(*mut GDBusConnection);

impl Connection {
    /// Create a new connection from a pointer.
    pub fn new(connection: *mut GDBusConnection) -> Self {
        Connection(connection)
    }

    /// Registers callbacks for exported objects at `object_path` with the D-Bus interface that is described in `interface_info` .
    /// Calls to functions in `vtable` (and `user_data_free_func`) will happen in the thread-default main context of the thread you are calling this method from.
    /// Note that all `GVariant` values passed to functions in `vtable` will match the signature given in `interface_info` - if a remote caller passes incorrect values, the `org.freedesktop.DBus.Error.InvalidArgs` is returned to the remote caller.
    /// Additionally, if the remote caller attempts to invoke methods or access properties not mentioned in `interface_info` the `org.freedesktop.DBus.Error.UnknownMethod` resp. `org.freedesktop.DBus.Error.InvalidArgs` errors are returned to the caller.
    /// It is considered a programming error if the `GDBusInterfaceGetPropertyFunc` function in `vtable` returns a `GVariant` of incorrect type.
    /// If an existing callback is already registered at `object_path` and `interface_name` , then `error` is set to `G_IO_ERROR_EXISTS`.
    /// GDBus automatically implements the standard D-Bus interfaces org.freedesktop.DBus.Properties, org.freedesktop.DBus.Introspectable and org.freedesktop.Peer, so you don't have to implement those for the objects you export. You can implement org.freedesktop.DBus.Properties yourself, e.g. to handle getting and setting of properties asynchronously.
    /// Note that the reference count on `interface_info` will be incremented by 1 (unless allocated statically, e.g. if the reference count is -1, see `g_dbus_interface_info_ref()`) for as long as the object is exported. Also note that `vtable` will be copied.
    /// See this server for an example of how to use this method.
    pub fn register_object<F: Fn(&str, Variant, &MethodInvocation) + 'static>(&self, object_path: &str, interface_info: InterfaceInfo, method_call_callback: F) {
        let object_path = CString::new(object_path).unwrap();
        let vtable = GDBusInterfaceVTable {
            method_call: handle_method_call,
            get_property: handle_get_property,
            set_property: handle_set_property,
        };
        let callback: Box<Box<Fn(&str, Variant, &MethodInvocation) + 'static>> = Box::new(Box::new(method_call_callback));
        unsafe { g_dbus_connection_register_object(self.0, object_path.as_ptr(), interface_info.to_glib(), &vtable as *const _, Box::into_raw(callback) as *mut _, user_data_free_func, null_mut()) };
    }

    /// Asynchronously sends `message` to the peer represented by `connection`.
    /// Unless `flags` contain the `G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL` flag, the serial number
    /// will be assigned by `connection` and set on `message` via `g_dbus_message_set_serial()`. If
    /// `out_serial` is not `NULL`, then the serial number used will be written to this location prior
    /// to submitting the message to the underlying transport.
    /// If `connection` is closed then the operation will fail with `G_IO_ERROR_CLOSED`. If `cancellable`
    /// is canceled, the operation will fail with `G_IO_ERROR_CANCELLED`. If `message` is not
    /// well-formed, the operation fails with `G_IO_ERROR_INVALID_ARGUMENT`.
    /// This is an asynchronous method. When the operation is finished, `callback` will be invoked in
    /// the thread-default main context of the thread you are calling this method from. You can
    /// then call `g_dbus_connection_send_message_with_reply_finish()` to get the result of the
    /// operation. See `g_dbus_connection_send_message_with_reply_sync()` for the synchronous
    /// version.
    /// Note that `message` must be unlocked, unless `flags` contain the
    /// `G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL` flag.
    /// See this server and client for an example of how to use this low-level API to send and
    /// receive UNIX file descriptors.
    pub fn send_message_with_reply<F: Fn(Result<Message, Error>) + 'static>(&self, message: Message, flags: SendMessageFlags, callback: F) {
        let callback: SendMessageCallback = Box::new(Box::new(callback));
        unsafe { g_dbus_connection_send_message_with_reply(self.0, message.to_glib(), GDBusSendMessageFlags::from_bits_truncate(flags.bits()), -1, null_mut(), null_mut(), send_message_callback, Box::into_raw(callback) as *mut _) };
    }

    /// Synchronously sends `message` to the peer represented by `connection` and blocks the calling thread until a reply is received or the timeout is reached. See `g_dbus_connection_send_message_with_reply()` for the asynchronous version of this method.
    /// Unless `flags` contain the `G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL` flag, the serial number will be assigned by `connection` and set on `message` via `g_dbus_message_set_serial()`. If `out_serial` is not `NULL`, then the serial number used will be written to this location prior to submitting the message to the underlying transport.
    /// If `connection` is closed then the operation will fail with `G_IO_ERROR_CLOSED`. If `cancellable` is canceled, the operation will fail with `G_IO_ERROR_CANCELLED`. If `message` is not well-formed, the operation fails with `G_IO_ERROR_INVALID_ARGUMENT`.
    /// Note that `error` is only set if a local in-process error occurred. That is to say that the returned `GDBusMessage` object may be of type `G_DBUS_MESSAGE_TYPE_ERROR`. Use `g_dbus_message_to_gerror()` to transcode this to a `GError`.
    /// See this server and client for an example of how to use this low-level API to send and receive UNIX file descriptors.
    /// Note that `message` must be unlocked, unless `flags` contain the `G_DBUS_SEND_MESSAGE_FLAGS_PRESERVE_SERIAL` flag.
    pub fn send_message_with_reply_sync(&self, message: Message, flags: SendMessageFlags) -> Result<Message, Error> {
        let mut error = null_mut();
        let message = unsafe { g_dbus_connection_send_message_with_reply_sync(self.0, message.to_glib(), GDBusSendMessageFlags::from_bits_truncate(flags.bits()), -1, null_mut(), null_mut(), &mut error) };
        message_to_result(message, error)
    }
}

fn message_to_result(message: *mut GDBusMessage, mut error: *mut GError) -> Result<Message, Error> {
    if error.is_null() {
        if unsafe { g_dbus_message_get_message_type(message) } == G_DBUS_MESSAGE_TYPE_ERROR {
            unsafe { g_dbus_message_to_gerror(message, &mut error) };
            Err(unsafe { from_glib_full(error) })
        }
        else {
            Ok(Message::new(message))
        }
    }
    else {
        Err(unsafe { from_glib_full(error) })
    }
}

unsafe extern fn handle_method_call(_connection: *mut GDBusConnection, _sender: *const c_char, _object_path: *const c_char, _interface_name: *const c_char, method_name: *const c_char, parameters: *mut GVariant, invocation: *mut GDBusMethodInvocation, user_data: *mut c_void) {
    let callback: &Box<Fn(&str, Variant, &MethodInvocation) + 'static> = &*(user_data as *const Box<_>);
    let cstring = CStr::from_ptr(method_name);
    callback(cstring.to_str().unwrap(), Variant::new(parameters), &MethodInvocation::new(invocation));
}

unsafe extern fn handle_get_property(_connection: *mut GDBusConnection, _sender: *const c_char, _object_path: *const c_char, _interface_name: *const c_char, _property_name: *const c_char, _error: *mut *mut GError, _user_data: *mut c_void) {
    // TODO
    println!("Get property");
}

unsafe extern fn handle_set_property(_connection: *mut GDBusConnection, _sender: *const c_char, _object_path: *const c_char, _interface_name: *const c_char, _property_name: *const c_char, _value: *mut GVariant, _error: *mut *mut GError, _user_data: *mut c_void) {
    // TODO
    println!("Set property");
}

unsafe extern fn send_message_callback(source_object: *mut GObject, res: *mut GAsyncResult, user_data: *mut c_void) {
    let mut error = null_mut();
    let message = g_dbus_connection_send_message_with_reply_finish(source_object as *mut _, res, &mut error);
    let result = message_to_result(message, error);
    let callback: &Box<Fn(Result<Message, Error>) + 'static> = &*(user_data as *const Box<_>);
    callback(result);
}
