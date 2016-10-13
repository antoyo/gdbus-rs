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

//! Watching Bus Names — Simple API for watching bus names.

use std::ffi::CString;

use gio_sys::{GBusNameWatcherFlags, GDBusConnection, g_bus_unwatch_name, g_bus_watch_name};
use glib::translate::ToGlib;
use libc::{c_char, c_void};

use connection::Connection;
use own_name::Type;

bitflags! {
    /// Flags used in `watch_name()`.
    pub flags NameWatcherFlags: u32 {
        /// No flags set.
        const NAME_WATCHER_FLAGS_NONE = 0,
        /// If no-one owns the name when beginning to watch the name, ask the bus to launch an owner for the name.
        const NAME_WATCHER_FLAGS_AUTO_START = 1,
    }
}

/// Watching Bus Names.
pub struct Watch {
    id: u32,
}

impl Watch {
    /// Starts watching `name` on the bus specified by `bus_type` and calls `name_appeared_handler` and `name_vanished_handler` when the name is known to have a owner respectively known to lose its owner. Callbacks will be invoked in the thread-default main context of the thread you are calling this function from.
    /// You are guaranteed that one of the handlers will be invoked after calling this function. When you are done watching the name, just call `g_bus_unwatch_name()` with the watcher id this function returns.
    /// If the name vanishes or appears (for example the application owning the name could restart), the handlers are also invoked. If the `GDBusConnection` that is used for watching the name disconnects, then `name_vanished_handler` is invoked since it is no longer possible to access the name.
    /// Another guarantee is that invocations of `name_appeared_handler` and `name_vanished_handler` are guaranteed to alternate; that is, if `name_appeared_handler` is invoked then you are guaranteed that the next time one of the handlers is invoked, it will be `name_vanished_handler` . The reverse is also true.
    /// This behavior makes it very simple to write applications that want to take action when a certain name exists. Basically, the application should create object proxies in `name_appeared_handler` and destroy them again (if any) in `name_vanished_handler`.
    pub fn name(bus_type: Type, name: &str, flags: NameWatcherFlags) -> WatchBuilder {
        WatchBuilder {
            bus_type: bus_type,
            name: name.to_string(),
            name_appeared_callback: None,
            flags: flags,
        }
    }
}

impl Drop for Watch {
    fn drop(&mut self) {
        unsafe { g_bus_unwatch_name(self.id) };
    }
}

/// `Watch` builder.
pub struct WatchBuilder {
    bus_type: Type,
    name: String,
    name_appeared_callback: Option<Box<Fn(&Connection, &str)>>,
    flags: NameWatcherFlags,
}

impl WatchBuilder {
    /// Create the watcher.
    pub fn build(self) -> Watch {
        let name = CString::new(self.name).unwrap();
        let callback: Box<Box<Fn(&Connection, &str) + 'static>> = Box::new(self.name_appeared_callback.unwrap());
        let id = unsafe { g_bus_watch_name(self.bus_type.to_glib(), name.as_ptr(), GBusNameWatcherFlags::from_bits_truncate(self.flags.bits()), Some(name_appeared_handler), Some(name_vanished_handler),
            Box::into_raw(callback) as *mut _, None
        )};
        Watch {
            id: id,
        }
    }

    /// Connect the name appeared event.
    pub fn connect_name_appeared<F: Fn(&Connection, &str) + 'static>(mut self, callback: F) -> Self {
        self.name_appeared_callback = Some(Box::new(callback));
        self
    }
}

unsafe extern "C" fn name_appeared_handler(connection: *mut GDBusConnection, _name: *const c_char, name_owner: *const c_char, user_data: *mut c_void) {
    let name_owner = CString::from_raw(name_owner as *mut _);
    let callback: &Box<Fn(&Connection, &str) + 'static> = &*(user_data as *const Box<_>);
    callback(&Connection::new(connection), &name_owner.into_string().unwrap());
}

unsafe extern "C" fn name_vanished_handler(_connection: *mut GDBusConnection, _name: *const c_char, _user_data: *mut c_void) {
    // TODO
}
