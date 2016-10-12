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

#[macro_use]
extern crate gdbus;
extern crate gio_sys;
extern crate glib_sys;
extern crate gtk;
extern crate libc;

use gdbus::connection::Connection;
use gdbus::method_invocation::MethodInvocation;
use gdbus::node_info::NodeInfo;
use gdbus::own_name::{OwnName, Type, NAME_OWNER_FLAGS_NONE};
use gdbus::variant::{FromVariant, Variant};

fn handle_method_call(method_name: &str, args: Variant, invocation: &MethodInvocation) {
    match method_name {
        "decrement_increment" => {
            let (number,): (i64,) = FromVariant::from_variant(&args);
            let decrement = number - 1;
            let increment = number as u8 + 1;
            invocation.return_value((decrement, increment));
        },
        _ => unreachable!(),
    }
}

const IN_ARG_GREETING: ::gio_sys::GDBusArgInfo = ::gio_sys::GDBusArgInfo {
    ref_count: ::glib_sys::Volatile(-1),
    name: c_str!("number"),
    signature: c_str!("x"),
    annotations: 0 as *mut _,
};

const IN_ARGS: [*const ::gio_sys::GDBusArgInfo; 2] = [&IN_ARG_GREETING, 0 as *const _];

const OUT_ARG_DECREMENT: ::gio_sys::GDBusArgInfo = ::gio_sys::GDBusArgInfo {
    ref_count: ::glib_sys::Volatile(-1),
    name: c_str!("decrement"),
    signature: c_str!("x"),
    annotations: 0 as *mut _,
};

const OUT_ARG_INCREMENT: ::gio_sys::GDBusArgInfo = ::gio_sys::GDBusArgInfo {
    ref_count: ::glib_sys::Volatile(-1),
    name: c_str!("increment"),
    signature: c_str!("y"),
    annotations: 0 as *mut _,
};

const OUT_ARGS: [*const ::gio_sys::GDBusArgInfo; 3] = [&OUT_ARG_DECREMENT, &OUT_ARG_INCREMENT, 0 as *const _];

const METHOD_DECREMENT_INCREMENT: ::gio_sys::GDBusMethodInfo = ::gio_sys::GDBusMethodInfo {
    ref_count: ::glib_sys::Volatile(-1),
    name: c_str!("decrement_increment"),
    in_args: &IN_ARGS as *const _ as *mut _,
    out_args: &OUT_ARGS as *const _ as *mut _,
    annotations: 0 as *mut _,
};

const METHODS: [*const ::gio_sys::GDBusMethodInfo; 2] = [&METHOD_DECREMENT_INCREMENT, 0 as *const _];

const INTERFACE: ::gio_sys::GDBusInterfaceInfo = ::gio_sys::GDBusInterfaceInfo {
    ref_count: ::glib_sys::Volatile(-1),
    name: c_str!("org.gtk.GDBus.TestInterface"),
    methods: &METHODS as *const _ as *mut _,
    signals: 0 as *mut _,
    properties: 0 as *mut _,
    annotations: 0 as *mut _,
};

const INTERFACES: [*const ::gio_sys::GDBusInterfaceInfo; 2] = [&INTERFACE, 0 as *const _];

const NODE: ::gio_sys::GDBusNodeInfo = ::gio_sys::GDBusNodeInfo {
    ref_count: ::glib_sys::Volatile(-1),
    path: 0 as *mut _,
    interfaces: &INTERFACES as *const _ as *mut _,
    nodes: 0 as *mut _,
    annotations: 0 as *mut _,
};

fn on_bus_acquired(connection: &Connection) {
    let introspection_data = NodeInfo::new(&mut NODE);
    connection.register_object("/org/gtk/GDBus/TestObject", introspection_data.interface(0), handle_method_call)
}

fn main() {
    gtk::init().unwrap();

    let _own_name = OwnName::new(Type::Session, "org.gtk.GDBus.TestServer", NAME_OWNER_FLAGS_NONE)
        .connect_bus_acquired(on_bus_acquired)
        .build();

    gtk::main();
}
