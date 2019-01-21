pub mod ole32;

use types::{
    HRESULT,
    IID,
    CLSID,
};
use std::{
    ops::Deref,
    os::raw::c_void,
};
use idl::com_interface;

#[com_interface(internal, iid = "00000000-0000-0000-0000-000000000046")]
pub struct IUnknown;

impl IUnknown {
    pub unsafe fn query_interface(
        &self,
        iid: &IID,
        ptr: *mut *mut c_void,
    ) -> HRESULT {
        ((*self.vtable).query_interface)(self as *const IUnknown, iid, ptr)
    }

    pub unsafe fn add_ref(&self) -> u32 {
        ((*self.vtable).add_ref)(self as *const IUnknown)
    }

    pub unsafe fn release(&self) -> u32 {
        ((*self.vtable).release)(self as *const IUnknown)
    }
}

#[repr(C)]
pub struct IUnknown_VTable {
    pub query_interface: extern "stdcall" fn(*const IUnknown, &IID, *mut *mut c_void) -> HRESULT,
    pub add_ref:         extern "stdcall" fn(*const IUnknown) -> u32,
    pub release:         extern "stdcall" fn(*const IUnknown) -> u32,
}



#[com_interface(internal, iid="00020400-0000-0000-c000-000000000046")]
pub struct IDispatch;

#[allow(non_snake_case)]
#[repr(C)]
pub struct IDispatch_VTable {
    __iunknown:       <IUnknown as ComInterface>::VTable,
    GetTypeInfoCount: *mut c_void, //extern "stdcall" fn(ComPtr, *mut u32) -> HRESULT,
    GetTypeInfo:      *mut c_void, //extern "stdcall" fn(ComPtr, u32, LCID, *mut *mut ITypeInfo) -> HRESULT,
    GetIDsOfNames:    *mut c_void, //extern "stdcall" fn(ComPtr, REFIID, *mut BSTR, u32, LCID, *mut DISPID) -> HRESULT,
    Invoke:           *mut c_void, //extern "stdcall" fn(ComPtr, DISPID, REFIID, LCID, WORD, *mut DISPPARAMS, *mut VARIANT, *mut EXCEPINFO, *mut u32) -> HRESULT,
}

pub mod types {

    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    #[repr(transparent)]
    pub struct DWORD(pub u32);

    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    #[repr(transparent)]
    pub struct HRESULT(pub u32);

    impl HRESULT {
        pub const S_OK: HRESULT = HRESULT(0);
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct IID([u8;16]);

    impl IID {
        pub const fn new(data: [u8; 16]) -> IID {
            IID(data)
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct CLSID(pub GUID);

    impl CLSID {
        pub const fn new(
            data1: u32,
            data2: u16,
            data3: u16,
            data4: [u8; 8],
        ) -> CLSID {
            CLSID(GUID {
                data1,
                data2,
                data3,
                data4,
            })
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[repr(C)]
    pub struct GUID {
        pub data1: u32,
        pub data2: u16,
        pub data3: u16,
        pub data4: [u8; 8],
    }
}

pub trait ComInterface {
    const IID: IID;
    type VTable;
    unsafe fn vtable(&self) -> *const Self::VTable;
}

pub trait ComClass {
    const CLSID: CLSID;
    type ClassInterface: ComInterface;
}

pub struct ComRef<T: ComInterface> {
    ptr: *const T,
}

impl<T: ComInterface> ComRef<T> {
    pub unsafe fn new(ptr: *const T) -> ComRef<T> {
        ComRef { ptr }
    }

    fn as_iunknown(&self) -> &IUnknown {
        unsafe { &*((self.ptr) as *const IUnknown) }
    }

    pub fn query_interface<U>(&self) -> Result<ComRef<U>, HRESULT>
    where
        U: ComInterface,
    {
        unsafe {
            let mut ptr: *mut c_void = std::mem::uninitialized();
            match self.as_iunknown().query_interface(&U::IID, &mut ptr) {
                HRESULT::S_OK => Ok(ComRef::new(ptr as *const U)),
                hr => Err(hr),
            }
        }
    }
}

impl<T: ComInterface> Deref for ComRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T: ComInterface> Clone for ComRef<T> {
    fn clone(&self) -> Self {
        unsafe {
            self.as_iunknown().add_ref();
        }
        ComRef { ptr: self.ptr }
    }
}

impl<T: ComInterface> Drop for ComRef<T> {
    fn drop(&mut self) {
        unsafe {
            self.as_iunknown().release();
        }
    }
}

