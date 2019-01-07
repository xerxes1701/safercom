#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro2::{Ident, Span};

type TokenStream = proc_macro::TokenStream;

use syn::{
    parse_macro_input,
    ItemStruct
};
use quote::quote;

#[proc_macro_attribute]
pub fn com_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    //let attr = syn::parse::parse (input).unwrap();
    let item = parse_macro_input!(item as ItemStruct);

    com_interface_impl(item).into()
}

fn com_interface_impl(item: ItemStruct) -> proc_macro2::TokenStream{
    let struct_name = item.ident;
    let vtable_name = Ident::new(
        &format!("{}_VTable", struct_name),
        Span::call_site());

    let gen = quote! {
        struct #struct_name {
            vtable: *const #vtable_name
        }

        impl ::safercom::ComInterface for #struct_name {
            type VTable = #vtable_name;

            const IID: ::safercom::types::IID = ::safercom::types::IID::new(
                0x0000000, 0x0000, 0x0000, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

            unsafe fn vtable(&self) -> *const Self::VTable {
                self.vtable
            }
        }
    };

    gen
}

#[test]
fn test(){

    let input: ItemStruct = syn::parse_quote!{
        pub struct Foo;
    };

    let actual = com_interface_impl(input);

    let expected = quote::quote! {
        struct Foo {
            vtable: *const Foo_VTable
        }

        impl ::safercom::ComInterface for Foo {
            type VTable = Foo_VTable;

            const IID: ::safercom::types::IID =
                ::safercom::types::IID::new(0x0000000, 0x0000, 0x0000, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

            unsafe fn vtable(&self) -> *const Self::VTable {
                self.vtable
            }
        }
    };

    assert_eq!(
        format!("{}", expected),
        format!("{}", actual));
}

