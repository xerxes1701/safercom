#![allow(dead_code)]

extern crate idl;
extern crate safercom;

use idl::com_interface;

#[com_interface(iid="11112222-3333-4444-55556666")]
struct Foo;

#[allow(non_camel_case_types)]
struct Foo_VTable;