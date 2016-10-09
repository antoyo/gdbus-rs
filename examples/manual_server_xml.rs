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

extern crate gdbus;
extern crate gtk;

use gdbus::connection::Connection;
use gdbus::method_invocation::MethodInvocation;
use gdbus::node_info::NodeInfo;
use gdbus::own_name::{OwnName, Type, NAME_OWNER_FLAGS_NONE};
use gdbus::variant::{FromVariant, Variant};

const INTROSPECTION_XML: &'static str = "<node>
  <interface name='org.gtk.GDBus.TestInterface'>
    <method name='DecrementIncrement'>
      <arg type='x' name='number' direction='in'/>
      <arg type='x' name='decrement' direction='out'/>
      <arg type='y' name='increment' direction='out'/>
    </method>
    <method name='Increment'>
      <arg type='x' name='number' direction='in'/>
      <arg type='x' name='response' direction='out'/>
    </method>
    <method name='IsTrue'>
      <arg type='b' name='boolean' direction='in'/>
      <arg type='b' name='response' direction='out'/>
    </method>
    <method name='HelloWorld'>
      <arg type='s' name='greeting' direction='in'/>
      <arg type='s' name='response' direction='out'/>
    </method>
    <method name='MultipleResults'>
      <arg type='x' name='number' direction='in'/>
      <arg type='n' name='result1' direction='out'/>
      <arg type='q' name='result2' direction='out'/>
      <arg type='i' name='result3' direction='out'/>
      <arg type='u' name='result4' direction='out'/>
      <arg type='t' name='result5' direction='out'/>
    </method>
  </interface>
</node>";

fn hello_world(greeting: &str) -> String {
    format!("You greeted me with '{}'. Thanks!", greeting)
}

fn handle_method_call(method_name: &str, args: Variant, invocation: &MethodInvocation) {
    match method_name {
        "DecrementIncrement" => {
            let (number,): (i64,) = FromVariant::from_variant(&args);
            let decrement = number - 1;
            let increment = number as u8 + 1;
            invocation.return_value((decrement, increment));
        },
        "HelloWorld" => {
            let (greeting,): (String,) = FromVariant::from_variant(&args);
            let response = hello_world(&greeting);
            invocation.return_value((response,));
        },
        "Increment" => {
            let (number,): (i64,) = FromVariant::from_variant(&args);
            let response = number + 1;
            invocation.return_value((response,));
        },
        "IsTrue" => {
            let (boolean,): (bool,) = FromVariant::from_variant(&args);
            invocation.return_value((boolean,));
        },
        "MultipleResults" => {
            let (number,): (i64,) = FromVariant::from_variant(&args);
            invocation.return_value((number as i16 - 2, number as u16 - 1, number as i32, number as u32 + 1, number as u64 + 2));
        },
        _ => unreachable!(),
    }
}

fn on_bus_acquired(connection: &Connection) {
    match NodeInfo::new_for_xml(INTROSPECTION_XML) {
        Ok(introspection_data) => {
            connection.register_object("/org/gtk/GDBus/TestObject", introspection_data.interface(0), handle_method_call)
        },
        Err(error) => println!("{}", error),
    }
}

fn main() {
    gtk::init().unwrap();

    let _own_name = OwnName::new(Type::Session, "org.gtk.GDBus.TestServer", NAME_OWNER_FLAGS_NONE)
        .connect_bus_acquired(on_bus_acquired)
        .build();

    gtk::main();
}
