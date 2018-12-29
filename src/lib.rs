use std::os::raw::c_void;

use crate::types::{raw, HRESULT, IID};

#[repr(C)]
pub struct IUnknown {
    vtable: *const IUnknownVTable,
}

impl IUnknown {
    pub unsafe fn query_interface(&self, iid: &IID, ptr: *mut *mut c_void) -> HRESULT {
        HRESULT(((*self.vtable).query_interface)(
            self as *const IUnknown,
            iid,
            ptr,
        ))
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
    pub query_interface:
        extern "stdcall" fn(*const IUnknown, &IID, *mut *mut c_void) -> raw::HRESULT,
    pub add_ref: extern "stdcall" fn(*const IUnknown) -> u32,
    pub release: extern "stdcall" fn(*const IUnknown) -> u32,
}

mod types {

    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    pub struct HRESULT(pub raw::HRESULT);

    #[derive(Clone, Debug, PartialEq, Eq)]
    #[repr(C)]
    pub struct IID {
        pub data1: u32,
        pub data2: u16,
        pub data3: u16,
        pub data4: [u8; 8],
    }

    pub mod raw {
        pub type HRESULT = u32;
    }
}

pub trait ComInterface {
    fn as_iunknown(&self) -> IUnknown;
    fn iid() -> &'static IID;
    unsafe fn new(ptr: *const c_void) -> Self;
}

pub struct ComRef<T: ComInterface> {
    ptr: *const T,
}

impl<T: ComInterface> ComRef<T> {
    pub fn as_iunknown(&self) -> IUnknown {
        unsafe { (*self.ptr).as_iunknown() }
    }

    pub fn query_interface<U>(&self) -> Result<U, HRESULT>
    where
        U: ComInterface,
    {
        unsafe {
            let mut ptr: *mut c_void = std::mem::uninitialized();
            match self.as_iunknown().query_interface(U::iid(), &mut ptr) {
                HRESULT(0) => Ok(U::new(ptr)),
                HRESULT(hr) => Err(HRESULT(hr)),
            }
        }
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
