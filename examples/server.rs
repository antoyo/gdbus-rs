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
    <annotation name='org.gtk.GDBus.Annotation' value='OnInterface'/>
    <annotation name='org.gtk.GDBus.Annotation' value='AlsoOnInterface'/>
    <method name='HelloWorld'>
      <annotation name='org.gtk.GDBus.Annotation' value='OnMethod'/>
      <arg type='s' name='greeting' direction='in'/>
      <arg type='s' name='response' direction='out'/>
    </method>
    <method name='EmitSignal'>
      <arg type='d' name='speed_in_mph' direction='in'>
        <annotation name='org.gtk.GDBus.Annotation' value='OnArg'/>
      </arg>
    </method>
    <method name='GimmeStdout'/>
    <signal name='VelocityChanged'>
      <annotation name='org.gtk.GDBus.Annotation' value='Onsignal'/>
      <arg type='d' name='speed_in_mph'/>
      <arg type='s' name='speed_as_string'>
        <annotation name='org.gtk.GDBus.Annotation' value='OnArg_NonFirst'/>
      </arg>
    </signal>
    <property type='s' name='FluxCapicitorName' access='read'>
      <annotation name='org.gtk.GDBus.Annotation' value='OnProperty'>
        <annotation name='org.gtk.GDBus.Annotation' value='OnAnnotation_YesThisIsCrazy'/>
      </annotation>
    </property>
    <property type='s' name='Title' access='readwrite'/>
    <property type='s' name='ReadingAlwaysThrowsError' access='read'/>
    <property type='s' name='WritingAlwaysThrowsError' access='readwrite'/>
    <property type='s' name='OnlyWritable' access='write'/>
    <property type='s' name='Foo' access='read'/>
    <property type='s' name='Bar' access='read'/>
  </interface>
</node>";

fn hello_world(greeting: &str) -> String {
    format!("You greeted me with '{}'. Thanks!", greeting)
}

fn handle_method_call(method_name: &str, args: Variant, invocation: &MethodInvocation) {
    if method_name == "HelloWorld" {
        let (greeting,): (String,) = FromVariant::from_variant(&args);
        let response = hello_world(&greeting);
        invocation.return_value((response,));
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
