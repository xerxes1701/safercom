pub mod ole32;

use crate::types::{
    HRESULT,
    IID,
};
use std::{
    ops::Deref,
    os::raw::c_void,
};

#[repr(C)]
pub struct IUnknown {
    vtable: *const IUnknownVTable,
}

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
pub struct IUnknownVTable {
    pub query_interface: extern "stdcall" fn(*const IUnknown, &IID, *mut *mut c_void) -> HRESULT,
    pub add_ref:         extern "stdcall" fn(*const IUnknown) -> u32,
    pub release:         extern "stdcall" fn(*const IUnknown) -> u32,
}

impl ComInterface for IUnknown {
    const IID: IID = IID::new(0x00000000, 0x0000, 0x0000, [0, 0, 0, 0, 0, 0, 0, 0x46]);

    type VTable = IUnknownVTable;

    unsafe fn vtable(&self) -> *const Self::VTable {
        self.vtable
    }
}

mod types {

    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    #[repr(C)]
    pub struct DWORD(pub u32);

    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    #[repr(C)]
    pub struct HRESULT(pub u32);

    impl HRESULT {
        pub const S_OK: HRESULT = HRESULT(0);
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[repr(C)]
    pub struct IID(pub GUID);

    impl IID {
        pub const fn new(
            data1: u32,
            data2: u16,
            data3: u16,
            data4: [u8; 8],
        ) -> IID {
            IID(GUID {
                data1,
                data2,
                data3,
                data4,
            })
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[repr(C)]
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