extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{
    parse_macro_input,
    ItemStruct
};
use quote::quote;

#[proc_macro_attribute]
pub fn com_interface(_attr: TokenStream, item: TokenStream) -> TokenStream {
    //let attr = syn::parse::parse (input).unwrap();
    let item = parse_macro_input!(item as ItemStruct);

    com_interface_impl(item)
}

fn com_interface_impl(item: ItemStruct) -> TokenStream{
    let name = item.ident;

    let gen = quote! {
        struct #name {
            vtable: *const Foo_VTable
        }

        impl ::safercom::ComInterface for #name {
            type VTable = Foo_VTable;

            const IID: ::safercom::types::IID = ::safercom::types::IID::new(
                0x0000000, 0x0000, 0x0000, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

            unsafe fn vtable(&self) -> *const Self::VTable {
                self.vtable
            }
        }
    };

    gen.into()
}