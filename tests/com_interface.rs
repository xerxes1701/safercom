#![allow(dead_code, unused_imports)]

extern crate idl;
extern crate safercom;

use idl::com_interface;

use safercom::{types::IID, ComInterface};

#[com_interface(iid="00112233-4455-6677-8899-aabbccddeeff")]
pub struct Foo;

#[allow(non_camel_case_types)]
pub struct Foo_VTable;

#[test]
fn foo_iid_test(){
    let expected_iid = safercom::types::IID::new(
        [0x33, 0x22, 0x11, 0x00, 0x55, 0x44, 0x77, 0x66, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]
    );
    let actual_iid = <Foo as ComInterface>::IID;
    
    assert_eq!(
        actual_iid,
        expected_iid
    );
}

#[test]
fn foo_vtable_pointer_test(){
    unsafe {
        let vtable = &Foo_VTable{};
        let foo = Foo{ vtable: vtable };
        assert!( foo.vtable() == vtable );
    }
}