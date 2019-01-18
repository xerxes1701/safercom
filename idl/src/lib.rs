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

    let iid = get_iid(&attr).unwrap();

    let gen = quote! {

        #[repr(C)]
        pub struct #struct_name {
            vtable: *const #vtable_name
        }

        impl ComInterface for #struct_name {
            type VTable = #vtable_name;

            const IID: IID = IID::new( #iid );

            unsafe fn vtable(&self) -> *const Self::VTable {
                self.vtable
            }
        }
    };

    gen
}

fn get_iid(args: &AttributeArgs) -> Result<proc_macro2::TokenStream, &'static str> {
    let attr = find_attr_value_by_name(args, "iid").ok_or("No IID parameter")?;
    to_guid_byte_array(attr)
}

fn find_attr_value_by_name<'a>(args: &'a AttributeArgs, name: &str) -> Option<&'a Lit> {
    for meta in args {
        if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { ident, lit, .. })) = meta {
            if format!("{}", ident) == name {
                return Some(lit);
            }
        }
    }
    return None;
}

fn to_guid_byte_array(attr: &Lit) -> Result<proc_macro2::TokenStream, &'static str> {
    if let Lit::Str(value) = attr {
        let guid = parse_guid_parts(&value.value())?;
        let guid = &guid;
        Ok(quote!([#(#guid),*]))
    } else {
        Err("value_not_a_string")
    }
}

fn parse_guid_parts(text: &str) -> Result<[u8;16], &'static str> {
    let re = regex::Regex::new("([[:xdigit:]]{8})-([[:xdigit:]]{4})-([[:xdigit:]]{4})-([[:xdigit:]]{4})-([[:xdigit:]]{12})").unwrap();
    let captures = re.captures(text).ok_or("GUID format '8-4-4-4-12' expected. eg.: '00112233-4455-6677-8899-aabbccddeeff'")?;

    //unwrap ok; can't fail due to regex constraint 'xdigit'
    let data1 = u32::from_str_radix(&captures[1], 16).unwrap();
    let data2 = u16::from_str_radix(&captures[2], 16).unwrap();
    let data3 = u16::from_str_radix(&captures[3], 16).unwrap();
    let data4 = u16::from_str_radix(&captures[4], 16).unwrap();
    let data5 = u64::from_str_radix(&captures[5], 16).unwrap();

    let mut data = [0u8; 16];
   
    data[0..4].clone_from_slice(&data1.to_le_bytes());
    data[4..6].clone_from_slice(&data2.to_le_bytes());
    data[6..8].clone_from_slice(&data3.to_le_bytes());
    data[8..10].clone_from_slice(&data4.to_be_bytes());
    data[10..16].clone_from_slice(&data5.to_be_bytes()[2..8]);

    Ok(data)
}


#[cfg(test)]
mod test {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn parse_guid_parts_test(){
        let parts = parse_guid_parts("00112233-4455-6677-8899-aabbccddeeff").unwrap();
        assert_eq!(
            parts,
            [0x33, 0x22, 0x11, 0x00, 0x55, 0x44, 0x77, 0x66, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    }

    #[test]
    fn get_iid_success_test() {
        let meta: NestedMeta = parse_quote!(iid = "00112233-4455-6677-8899-aabbccddeeff");
        let actual = get_iid(&vec![meta]).unwrap();

        let expected = quote!([51u8, 34u8, 17u8, 0u8, 85u8, 68u8, 119u8, 102u8, 136u8, 153u8, 170u8, 187u8, 204u8, 221u8, 238u8, 255u8]);

        assert_eq!(format!("{}", expected), format!("{}", actual));
    }

    #[test]
    fn get_iid_failure_wrong_name_test() {
        let meta: NestedMeta = parse_quote!(foo = "d8f015c0-c278-11ce-a49e-444553540000");

        assert_eq!(
            get_iid(&vec![meta]).expect_err("should fail"),
            "No IID parameter");
    }

    #[test]
    fn get_iid_failure_value_not_a_string_test() {
        let meta: NestedMeta = parse_quote!(iid = 3454325);

        assert_eq!(
            get_iid(&vec![meta]).expect_err("should fail"),
            "value_not_a_string");
    }

    #[test]
    fn com_interface_impl_test() {
        let attr = {
            let iid: NestedMeta = parse_quote!(iid = "00112233-4455-6677-8899-aabbccddeeff");
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
                    ::safercom::types::IID::new([51u8, 34u8, 17u8, 0u8, 85u8, 68u8, 119u8, 102u8, 136u8, 153u8, 170u8, 187u8, 204u8, 221u8, 238u8, 255u8]);

                unsafe fn vtable(&self) -> *const Self::VTable {
                    self.vtable
                }
            }
        };

        assert_eq!(format!("{}", expected), format!("{}", actual));
    }
}
