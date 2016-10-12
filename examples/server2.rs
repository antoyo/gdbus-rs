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

#![allow(non_upper_case_globals)]

#[macro_use]
extern crate gdbus;
extern crate gio_sys;
extern crate glib_sys;
extern crate gtk;

use gdbus::method_invocation::MethodInvocation;
use gdbus::node_info::NodeInfo;
use gdbus::own_name::{OwnName, Type, NAME_OWNER_FLAGS_NONE};
use gdbus::variant::Variant;

dbus_class!("org.gtk.GDBus.TestInterface", class TestClass (number: i64) {
    fn get_number(&this) -> i64 {
        this.number
    }
});

fn main() {
    gtk::init().unwrap();

    let mut test_object = TestClass::new("org.gtk.GDBus.TestServer", 42);
    test_object.run("/org/gtk/GDBus/TestObject");

    gtk::main();
}
