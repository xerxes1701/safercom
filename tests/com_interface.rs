#![allow(dead_code)]

extern crate idl;
extern crate safercom;

use idl::com_interface;

#[com_interface]
struct Foo;

#[allow(non_camel_case_types)]
struct Foo_VTable;