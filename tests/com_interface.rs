#![allow(dead_code)]

extern crate idl;
extern crate safercom;

use idl::com_interface;
use safercom::{ComInterface, types::{IID}};

#[com_interface(iid="11112222-3333-4444-5555-666677778888")]
pub struct Foo;

#[allow(non_camel_case_types)]
pub struct Foo_VTable;