#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro2::{
    Ident,
    Span,
};

type TokenStream = proc_macro::TokenStream;

use quote::quote;
use syn::{
    parse_macro_input,
    AttributeArgs,
    ItemStruct,
    Lit,
    Meta,
    MetaNameValue,
    NestedMeta,
};

#[proc_macro_attribute]
pub fn com_interface(
    attr: TokenStream,
    item: TokenStream,
) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let attr = parse_macro_input!(attr as AttributeArgs);

    com_interface_impl(attr, item).into()
}

fn com_interface_impl(
    attr: AttributeArgs,
    item: ItemStruct,
) -> proc_macro2::TokenStream {
    let struct_name = item.ident;
    let vtable_name = Ident::new(&format!("{}_VTable", struct_name), Span::call_site());

    let _iid = get_iid(&attr);

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

fn get_iid(args: &AttributeArgs) -> Result<String, &'static str> {
    for meta in args {
        if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            ident: name, lit, ..
        })) = meta
        {
            if format!("{}", name) == "iid" {
                if let Lit::Str(value) = lit {
                    return Ok(value.value());
                } else {
                    return Err("value_not_a_string");
                }
            }
        }
    }
    Err("No IID parameter")
}

#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn get_iid_success_test() {
        let meta: NestedMeta = parse_quote!(iid = "d8f015c0-c278-11ce-a49e444553540000");

        assert_eq!(
            get_iid(&vec![meta]),
            Ok("d8f015c0-c278-11ce-a49e444553540000".to_string())
        );
    }

    #[test]
    fn get_iid_failure_wrong_name_test() {
        let meta: NestedMeta = parse_quote!(foo = "d8f015c0-c278-11ce-a49e444553540000");

        assert_eq!(get_iid(&vec![meta]), Err("No IID parameter"));
    }

    #[test]
    fn get_iid_failure_value_not_a_string_test() {
        let meta: NestedMeta = parse_quote!(iid = 3454325);

        assert_eq!(get_iid(&vec![meta]), Err("value_not_a_string"));
    }

    #[test]
    fn com_interface_impl_test() {
        let attr = {
            let iid: NestedMeta = parse_quote!(iid = "d8f015c0-c278-11ce-a49e444553540000");
            vec![iid]
        };

        let item: ItemStruct = syn::parse_quote! {
            pub struct Foo;
        };

        let actual = com_interface_impl(attr, item);

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

        assert_eq!(format!("{}", expected), format!("{}", actual));
    }
}
