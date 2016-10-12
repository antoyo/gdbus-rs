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
    fn decrement_increment(number: i64) -> (i64, u8);
    fn hello_world(greeting: &str) -> String;
    fn increment(number: i64) -> i64;
    fn is_true(boolean: bool) -> bool;
    fn log(message: &str);
    fn log_default();
    fn multiple_results(number: i64) -> (i16, u16, i32, u32, u64);
}
);

fn main() {
    gtk::init().unwrap();

    let test_object = TestClass::new("org.gtk.GDBus.TestServer", "/org/gtk/GDBus/TestObject");
    if let Err(error) = test_object.is_true(true) {
        println!("Error: {}", error);
    }
    println!("decrement_increment(41): {:?}", test_object.decrement_increment(41).unwrap());
    println!("hello_world(\"Me\"): {}", test_object.hello_world("Me").unwrap());
    println!("is_true(true): {}", test_object.is_true(true).unwrap());
    println!("increment(41): {}", test_object.increment(41).unwrap());
    println!("multiple_results(41): {:?}", test_object.multiple_results(41).unwrap());
    test_object.log("Test Log Message").ok();
    test_object.log_default().ok();

    gtk::main();
}
