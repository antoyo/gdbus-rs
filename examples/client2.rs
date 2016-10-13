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
extern crate glib;
extern crate gtk;

dbus_interface!(
#[dbus("org.gtk.GDBus.TestInterface")]
interface TestClass {
    fn get_number(&self) -> i64;
    fn get_number_plus_x(&self, x: i64) -> i64;
    fn increment(&mut self);
    fn increment_by(&mut self, x: i64);
}
);

fn main() {
    gtk::init().unwrap();

    let test_object = TestClass::new("org.gtk.GDBus.TestServer", "/org/gtk/GDBus/TestObject");
    println!("get_number(): {}", test_object.get_number().unwrap());
    println!("get_number_plus_x(10): {}", test_object.get_number_plus_x(10).unwrap());
    test_object.increment().ok();
    println!("get_number(): {}", test_object.get_number().unwrap());
    test_object.increment_by(-1).ok();
    println!("get_number(): {}", test_object.get_number().unwrap());

    gtk::main();
}
