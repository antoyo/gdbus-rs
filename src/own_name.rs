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

//! Owning Bus Names — Simple API for owning bus names.

use std::ffi::CString;

use gio_sys::{GBusNameOwnerFlags, GBusType, GDBusConnection, g_bus_own_name, g_bus_unown_name, G_BUS_TYPE_STARTER, G_BUS_TYPE_NONE, G_BUS_TYPE_SYSTEM, G_BUS_TYPE_SESSION};
use glib::translate::ToGlib;
use libc::{c_char, c_void};

use connection::Connection;

bitflags! {
    /// Flags used in `OwnName::new()`.
    pub flags NameOwnerFlags: u32 {
        /// No flags set.
        const NAME_OWNER_FLAGS_NONE = 0,
        /// Allow another message bus connection to claim the name.
        const NAME_OWNER_FLAGS_ALLOW_REPLACEMENT = 1,
        /// If another message bus connection owns the name and have specified `G_BUS_NAME_OWNER_FLAGS_ALLOW_REPLACEMENT`, then take the name from the other connection.
        const NAME_OWNER_FLAGS_REPLACE = 2,
    }
}

/// Owning Bus Names.
pub struct OwnName {
    id:  u32,
}

impl OwnName {
    /// Starts acquiring `name` on the bus specified by `bus_type` and calls `name_acquired_handler` and `name_lost_handler` when the name is acquired respectively lost. Callbacks will be invoked in the thread-default main context of the thread you are calling this function from.
    /// You are guaranteed that one of the `name_acquired_handler` and `name_lost_handler` callbacks will be invoked after calling this function - there are three possible cases:
    /// `name_lost_handler` with a `NULL` connection (if a connection to the bus can't be made).
    /// `bus_acquired_handler` then `name_lost_handler` (if the name can't be obtained)
    /// `bus_acquired_handler` then `name_acquired_handler` (if the name was obtained).
    /// If the name is acquired or lost (for example another application could acquire the name if you allow replacement or the application currently owning the name exits), the handlers are also invoked. If the `GDBusConnection` that is used for attempting to own the name closes, then `name_lost_handler` is invoked since it is no longer possible for other processes to access the process.
    /// You cannot use `g_bus_own_name()` several times for the same name (unless interleaved with calls to `g_bus_unown_name()`) - only the first call will work.
    /// Another guarantee is that invocations of `name_acquired_handler` and `name_lost_handler` are guaranteed to alternate; that is, if `name_acquired_handler` is invoked then you are guaranteed that the next time one of the handlers is invoked, it will be `name_lost_handler`. The reverse is also true.
    /// If you plan on exporting objects (using e.g. `g_dbus_connection_register_object()`), note that it is generally too late to export the objects in `name_acquired_handler`. Instead, you can do this in `bus_acquired_handler` since you are guaranteed that this will run before `name` is requested from the bus.
    /// This behavior makes it very simple to write applications that wants to own names and export objects. Simply register objects to be exported in `bus_acquired_handler` and unregister the objects (if any) in `name_lost_handler`.
    pub fn new(bus_type: Type, name: &str, flags: NameOwnerFlags) -> OwnNameBuilder {
        OwnNameBuilder {
            bus_acquired_callback: None,
            bus_type: bus_type,
            flags: flags,
            name: name.to_string(),
        }
    }

    #[doc(hidden)]
    pub fn from_id(id: u32) -> OwnName {
        OwnName {
            id: id,
        }
    }
}

impl Drop for OwnName {
    fn drop(&mut self) {
        unsafe { g_bus_unown_name(self.id) };
    }
}

/// `OwnName` builder
pub struct OwnNameBuilder {
    bus_acquired_callback: Option<Box<Fn(&Connection)>>,
    bus_type: Type,
    flags: NameOwnerFlags,
    name: String,
}

impl OwnNameBuilder {
    /// Create the owning bus name.
    pub fn build(self) -> OwnName {
        let id =
            unsafe {
                let name = CString::new(self.name).unwrap();
                let callback: Box<Box<Fn(&Connection) + 'static>> = Box::new(self.bus_acquired_callback.unwrap());
                g_bus_own_name(self.bus_type.to_glib(), name.as_ptr(),
                    GBusNameOwnerFlags::from_bits_truncate(self.flags.bits()), Some(bus_acquired_handler),
                    Some(name_acquired_handler), Some(name_lost_handler),
                    Box::into_raw(callback) as *mut _, None
                )
            };
        OwnName {
            id: id,
        }
    }

    /// Connect the bus acquired event.
    pub fn connect_bus_acquired<F: Fn(&Connection) + 'static>(mut self, callback: F) -> Self {
        self.bus_acquired_callback = Some(Box::new(callback));
        self
    }
}

/// An enumeration for well-known message buses.
pub enum Type {
    /// An alias for the message bus that activated the process, if any.
    Starter,
    /// Not a message bus.
    None,
    /// The system-wide message bus.
    System,
    /// The login session message bus.
    Session,
}

#[doc(hidden)]
impl ToGlib for Type {
    type GlibType = GBusType;

    fn to_glib(&self) -> Self::GlibType {
        match *self {
            Type::Starter => G_BUS_TYPE_STARTER,
            Type::None => G_BUS_TYPE_NONE,
            Type::System => G_BUS_TYPE_SYSTEM,
            Type::Session => G_BUS_TYPE_SESSION,
        }
    }
}

unsafe extern "C" fn bus_acquired_handler(connection: *mut GDBusConnection, _name: *const c_char, user_data: *mut c_void) {
    let callback: &Box<Fn(&Connection) + 'static> = &*(user_data as *const Box<_>);
    callback(&Connection::new(connection));
}

unsafe extern "C" fn name_acquired_handler(_connection: *mut GDBusConnection, _name: *const c_char, _user_data: *mut c_void) {
    // TODO
    println!("Name acquired");
}

unsafe extern "C" fn name_lost_handler(_connection: *mut GDBusConnection, _name: *const c_char, _user_data: *mut c_void) {
    // TODO
    println!("Name lost");
}
