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

//! Convenient macros to create `DBus` client and server.

#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => { $sub };
}

#[macro_export]
macro_rules! c_str {
    ($($string:expr),*) => {
        concat!($($string),*, "\0") as *const _ as *mut _
    };
}

#[macro_export]
macro_rules! c_stringify {
    ($string:ident) => {
        c_str!(stringify!($string))
    };
}

#[macro_export]
macro_rules! dbus_prototypes {
    ($interface_name:expr,) => {};
    ($interface_name:expr, fn $func_name:ident () -> ( $($return_type:ty),* ) ; $($rest:tt)* ) => {
        pub fn $func_name(&self) -> Result<($($return_type),*), ::glib::error::Error> {
            let method_call_message = ::gdbus::message::Message::new_method_call(&self.dbus_name, &self.object_path, $interface_name, stringify!($func_name));
            self.connection.send_message_with_reply_sync(method_call_message, ::gdbus::connection::SEND_MESSAGE_FLAGS_NONE)
                .map(|message| {
                    let response: ($($return_type),*) = ::gdbus::variant::FromVariant::from_variant(&message.get_body());
                    response
                })
        }
        dbus_prototypes!($interface_name, $($rest)*);
    };
    ($interface_name:expr, fn $func_name:ident () -> $return_type:ty ; $($rest:tt)* ) => {
        pub fn $func_name(&self) -> Result<$return_type, ::glib::error::Error> {
            let method_call_message = ::gdbus::message::Message::new_method_call(&self.dbus_name, &self.object_path, $interface_name, stringify!($func_name));
            self.connection.send_message_with_reply_sync(method_call_message, ::gdbus::connection::SEND_MESSAGE_FLAGS_NONE)
                .map(|message| {
                    let (response,): ($return_type,) = ::gdbus::variant::FromVariant::from_variant(&message.get_body());
                    response
                })
        }
        dbus_prototypes!($interface_name, $($rest)*);
    };
    ($interface_name:expr, fn $func_name:ident () ; $($rest:tt)* ) => {
        pub fn $func_name(&self) -> Result<(), ::glib::error::Error> {
            let method_call_message = ::gdbus::message::Message::new_method_call(&self.dbus_name, &self.object_path, $interface_name, stringify!($func_name));
            self.connection.send_message(method_call_message, ::gdbus::connection::SEND_MESSAGE_FLAGS_NONE)
        }
        dbus_prototypes!($interface_name, $($rest)*);
    };
    ($interface_name:expr, fn $func_name:ident ($($arg:ident : $($arg_type:tt)*),*) -> ( $($return_type:ty),* ) ; $($rest:tt)* ) => {
        pub fn $func_name(&self, $($arg : $($arg_type)*),*) -> Result<($($return_type),*), ::glib::error::Error> {
            let method_call_message = ::gdbus::message::Message::new_method_call(&self.dbus_name, &self.object_path, $interface_name, stringify!($func_name));
            method_call_message.set_body(($($arg,)*));
            self.connection.send_message_with_reply_sync(method_call_message, ::gdbus::connection::SEND_MESSAGE_FLAGS_NONE)
                .map(|message| {
                    let response: ($($return_type),*) = ::gdbus::variant::FromVariant::from_variant(&message.get_body());
                    response
                })
        }
        dbus_prototypes!($interface_name, $($rest)*);
    };
    ($interface_name:expr, fn $func_name:ident ($($arg:ident : $($arg_type:tt)*),*) -> $return_type:ty ; $($rest:tt)* ) => {
        pub fn $func_name(&self, $($arg : $($arg_type)*),*) -> Result<$return_type, ::glib::error::Error> {
            let method_call_message = ::gdbus::message::Message::new_method_call(&self.dbus_name, &self.object_path, $interface_name, stringify!($func_name));
            method_call_message.set_body(($($arg,)*));
            self.connection.send_message_with_reply_sync(method_call_message, ::gdbus::connection::SEND_MESSAGE_FLAGS_NONE)
                .map(|message| {
                    let (response,): ($return_type,) = ::gdbus::variant::FromVariant::from_variant(&message.get_body());
                    response
                })
        }
        dbus_prototypes!($interface_name, $($rest)*);
    };
    ($interface_name:expr, fn $func_name:ident ($($arg:ident : $($arg_type:tt)*),*) ; $($rest:tt)* ) => {
        pub fn $func_name(&self, $($arg : $($arg_type)*),*) -> Result<(), ::glib::error::Error> {
            let method_call_message = ::gdbus::message::Message::new_method_call(&self.dbus_name, &self.object_path, $interface_name, stringify!($func_name));
            method_call_message.set_body(($($arg,)*));
            self.connection.send_message(method_call_message, ::gdbus::connection::SEND_MESSAGE_FLAGS_NONE)
        }
        dbus_prototypes!($interface_name, $($rest)*);
    };
}

#[macro_export]
macro_rules! dbus_interface {
    (# [ dbus ( $interface_name:expr ) ] interface $class_name:ident { $($prototypes:tt)* }) => {
        pub struct $class_name {
            connection: ::gdbus::connection::Connection,
            dbus_name: String,
            object_path: String,
        }

        impl $class_name {
            pub fn new(dbus_name: &str, object_path: &str) -> Self {
                let connection =
                    unsafe {
                        ::gio_sys::g_bus_get_sync(::gio_sys::G_BUS_TYPE_SESSION, ::std::ptr::null_mut(), ::std::ptr::null_mut())
                    };
                let connection = ::gdbus::connection::Connection::new(connection);
                $class_name {
                    connection: connection,
                    dbus_name: dbus_name.to_string(),
                    object_path: object_path.to_string(),
                }
            }

            dbus_prototypes!($interface_name, $($prototypes)*);
        }
    };
}

#[macro_export]
macro_rules! dbus_functions {
    ($method_name:expr, $args:expr, $invocation:expr,) => {
    };
    ($method_name:expr, $args:expr, $invocation:expr, fn $func_name:ident () -> () $block:block $($rest:tt)*) => {
        if $method_name == stringify!($func_name) {
            let _result = $block;
        }
        dbus_functions!($method_name, $args, $invocation, $($rest)*);
    };
    ($method_name:expr, $args:expr, $invocation:expr, fn $func_name:ident () -> ($($return_type:ty),*) $block:block $($rest:tt)*) => {
        if $method_name == stringify!($func_name) {
            let result: ($($return_type),*) = $block;
            $invocation.return_value(result);
        }
        dbus_functions!($method_name, $args, $invocation, $($rest)*);
    };
    ($method_name:expr, $args:expr, $invocation:expr, fn $func_name:ident () -> $return_type:ty $block:block $($rest:tt)*) => {
        if $method_name == stringify!($func_name) {
            let result: $return_type = $block;
            $invocation.return_value((result,));
        }
        dbus_functions!($method_name, $args, $invocation, $($rest)*);
    };
    ($method_name:expr, $args:expr, $invocation:expr, fn $func_name:ident ($($arg:ident : $arg_type:ty),*) -> () $block:block $($rest:tt)*) => {
        if $method_name == stringify!($func_name) {
            let ($($arg,)*): ($($arg_type,)*) = FromVariant::from_variant(&$args);
            let _result = $block;
        }
        dbus_functions!($method_name, $args, $invocation, $($rest)*);
    };
    ($method_name:expr, $args:expr, $invocation:expr, fn $func_name:ident ($($arg:ident : $arg_type:ty),*) -> ($($return_type:ty),*) $block:block $($rest:tt)*) => {
        if $method_name == stringify!($func_name) {
            let ($($arg,)*): ($($arg_type,)*) = FromVariant::from_variant(&$args);
            let result: ($($return_type),*) = $block;
            $invocation.return_value(result);
        }
        dbus_functions!($method_name, $args, $invocation, $($rest)*);
    };
    ($method_name:expr, $args:expr, $invocation:expr, fn $func_name:ident ($($arg:ident : $arg_type:ty),*) -> $return_type:ty $block:block $($rest:tt)*) => {
        if $method_name == stringify!($func_name) {
            let ($($arg,)*): ($($arg_type,)*) = FromVariant::from_variant(&$args);
            let result: $return_type = $block;
            $invocation.return_value((result,));
        }
        dbus_functions!($method_name, $args, $invocation, $($rest)*);
    };
}

#[macro_export]
macro_rules! dbus_arg_signature {
    (bool) => { "b" };
    (u8) => { "y" };
    (i16) => { "n" };
    (u16) => { "q" };
    (i32) => { "i" };
    (u32) => { "u" };
    (i64) => { "x" };
    (u64) => { "t" };
    (&str) => { "s" };
    (String) => { "s" };
}

#[macro_export]
macro_rules! dbus_methods {
    () => {
    };
    (fn $func_name:ident ($($arg:ident : $arg_type:tt),*) -> ($($return_type:tt),*) $block:block $($rest:tt)*) => {
        const $func_name: *mut ::gio_sys::GDBusMethodInfo = {
            $(
            const $arg: ::gio_sys::GDBusArgInfo = ::gio_sys::GDBusArgInfo {
                ref_count: ::glib_sys::Volatile(-1),
                name: c_stringify!($arg),
                signature: c_str!(dbus_arg_signature!($arg_type)),
                annotations: 0 as *mut _,
            };
            )*

            const IN_ARGS: [*mut ::gio_sys::GDBusArgInfo; dbus_count_idents!($($arg),*) + 1] = [$(&$arg as *const _ as *mut _,)* 0 as *mut _];

            const OUT_ARGS: [*mut ::gio_sys::GDBusArgInfo; dbus_count_idents!($($return_type),*) + 1] = [
                $(
                &::gio_sys::GDBusArgInfo {
                    ref_count: ::glib_sys::Volatile(-1),
                    name: c_str!("result", stringify!($return_type)),
                    signature: c_str!(dbus_arg_signature!($return_type)),
                    annotations: 0 as *mut _,
                } as *const _ as *mut _,
                )*
            0 as *mut _];

            &::gio_sys::GDBusMethodInfo {
                ref_count: ::glib_sys::Volatile(-1),
                name: c_stringify!($func_name),
                in_args: &IN_ARGS as *const _ as *mut _,
                out_args: &OUT_ARGS as *const _ as *mut _,
                annotations: 0 as *mut _,
            } as *const _ as *mut _
        };

        dbus_methods!($($rest)*);
    };
    (fn $func_name:ident ($($arg:ident : $arg_type:tt),*) -> $return_type:tt $block:block $($rest:tt)*) => {
        const $func_name: *mut ::gio_sys::GDBusMethodInfo = {
            $(
            const $arg: ::gio_sys::GDBusArgInfo = ::gio_sys::GDBusArgInfo {
                ref_count: ::glib_sys::Volatile(-1),
                name: c_stringify!($arg),
                signature: c_str!(dbus_arg_signature!($arg_type)),
                annotations: 0 as *mut _,
            };
            )*

            const IN_ARGS: [*mut ::gio_sys::GDBusArgInfo; dbus_count_idents!($($arg),*) + 1] = [$(&$arg as *const _ as *mut _,)* 0 as *mut _];

            const OUT_ARG: ::gio_sys::GDBusArgInfo = ::gio_sys::GDBusArgInfo {
                ref_count: ::glib_sys::Volatile(-1),
                name: c_str!("result"),
                signature: c_str!(dbus_arg_signature!($return_type)),
                annotations: 0 as *mut _,
            };

            const OUT_ARGS: [*mut ::gio_sys::GDBusArgInfo; 2] = [&OUT_ARG as *const _ as *mut _, 0 as *mut _];

            &::gio_sys::GDBusMethodInfo {
                ref_count: ::glib_sys::Volatile(-1),
                name: c_stringify!($func_name),
                in_args: &IN_ARGS as *const _ as *mut _,
                out_args: &OUT_ARGS as *const _ as *mut _,
                annotations: 0 as *mut _,
            } as *const _ as *mut _
        };

        dbus_methods!($($rest)*);
    };
    (fn $func_name:ident ($($arg:ident : $arg_type:tt),*) $block:block $($rest:tt)*) => {
        const $func_name: *mut ::gio_sys::GDBusMethodInfo = {
            $(
            const $arg: ::gio_sys::GDBusArgInfo = ::gio_sys::GDBusArgInfo {
                ref_count: ::glib_sys::Volatile(-1),
                name: c_stringify!($arg),
                signature: c_str!(dbus_arg_signature!($arg_type)),
                annotations: 0 as *mut _,
            };
            )*

            const IN_ARGS: [*mut ::gio_sys::GDBusArgInfo; dbus_count_idents!($($arg),*) + 1] = [$(&$arg as *const _ as *mut _,)* 0 as *mut _];

            const OUT_ARGS: [*mut ::gio_sys::GDBusArgInfo; 1] = [0 as *mut _];

            &::gio_sys::GDBusMethodInfo {
                ref_count: ::glib_sys::Volatile(-1),
                name: c_stringify!($func_name),
                in_args: &IN_ARGS as *const _ as *mut _,
                out_args: &OUT_ARGS as *const _ as *mut _,
                annotations: 0 as *mut _,
            } as *const _ as *mut _
        };

        dbus_methods!($($rest)*);
    };
}

#[macro_export]
macro_rules! dbus_count_idents {
    ($( $name:ident ),*) => {
        0usize
        $(+ {
            replace_expr!($name 1usize)
        })*
    };
}

#[macro_export]
macro_rules! dbus_function_names {
    ($(fn $func_name:ident ( $($tt:tt)* ) -> $return_type:ty $block:block)*) => {
        [$($func_name),*, 0 as *mut _]
    };
}

#[macro_export]
macro_rules! dbus_count_methods {
    () => { 0usize };
    (fn $func_name:ident ( $($tt:tt)* ) -> $return_type:ty $block:block $($rest:tt)*) => {
        replace_expr!($func_name 1usize)
            + dbus_count_methods!($($rest)*)
    };
    (fn $func_name:ident ( $($tt:tt)* ) $block:block $($rest:tt)*) => {
        replace_expr!($func_name 1usize)
            + dbus_count_methods!($($rest)*)
    };
}

#[macro_export]
macro_rules! dbus_class {
    ($interface_name:expr, class $class_name:ident { $($functions:tt)* }) => {
        pub struct $class_name {
            dbus_name: String,
            own_name: OwnName,
        }

        impl $class_name {
            pub fn new(dbus_name: &str) -> Self {
                $class_name {
                    dbus_name: dbus_name.to_string(),
                    own_name: OwnName::from_id(0),
                }
            }

            fn handle_method_call(method_name: &str, args: Variant, invocation: &MethodInvocation) {
                dbus_functions!(method_name, args, invocation, $($functions)*);
            }

            fn run(&mut self, bus_name: &str) {
                dbus_methods!($($functions)*);
                const METHODS: [*mut ::gio_sys::GDBusMethodInfo; dbus_count_methods!($($functions)*) + 1usize] = dbus_function_names!($($functions)*);

                const INTERFACE: ::gio_sys::GDBusInterfaceInfo = ::gio_sys::GDBusInterfaceInfo {
                    ref_count: ::glib_sys::Volatile(-1),
                    name: c_str!($interface_name),
                    methods: &METHODS as *const _ as *mut _,
                    signals: 0 as *mut _,
                    properties: 0 as *mut _,
                    annotations: 0 as *mut _,
                };

                const INTERFACES: [*mut ::gio_sys::GDBusInterfaceInfo; 2] = [&INTERFACE as *const _ as *mut _, 0 as *mut _];

                const NODE: ::gio_sys::GDBusNodeInfo = ::gio_sys::GDBusNodeInfo {
                    ref_count: ::glib_sys::Volatile(-1),
                    path: 0 as *mut _,
                    interfaces: &INTERFACES as *const _ as *mut _,
                    nodes: 0 as *mut _,
                    annotations: 0 as *mut _,
                };

                let bus_name = bus_name.to_string();
                let old = ::std::mem::replace(&mut self.own_name, OwnName::new(Type::Session, &self.dbus_name, NAME_OWNER_FLAGS_NONE)
                    .connect_bus_acquired(move |connection| {
                        let introspection_data = NodeInfo::new(&mut NODE);
                        connection.register_object(&bus_name, introspection_data.interface(0), $class_name::handle_method_call)
                    })
                    .build());
                ::std::mem::forget(old);

            }
        }
    };
    ($interface_name:expr, class $class_name:ident ($($variables:ident : $variable_types:ty),*) { $($functions:tt)* }) => {
        #[derive(Clone)]
        pub struct $class_name {
            $($variables : $variable_types,)*
        }

        impl $class_name {
            pub fn new($($variables: $variable_types),*) -> Self {
                $class_name {
                    $($variables : $variables,)*
                }
            }

            pub fn run(&self, bus_name: &str) {
                let connection = dbus::Connection::get_private(dbus::BusType::Session).unwrap();
                connection.register_name(bus_name, dbus::NameFlag::ReplaceExisting as u32).unwrap();

                let factory = dbus::tree::Factory::new_fn::<()>();
                let class = factory.tree().add(factory.object_path(format!("/{}", stringify!($class_name)), ()).introspectable().add({
                    let interface = factory.interface($interface_name, ());
                    dbus_functions!(self, factory, interface, $($functions)*);
                    interface
                }));
                class.set_registered(&connection, true).unwrap();

                for _ in class.run(&connection, connection.iter(1000)) {
                }
            }
        }
    };
}
