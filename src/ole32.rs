use crate::{
    types::{
        CLSID,
        DWORD,
        HRESULT,
        IID,
    },
    ComInterface,
    ComRef,
    IUnknown,
};
use std::{
    ffi::c_void,
    mem::transmute,
    ptr::null_mut,
};

pub fn co_initialize() -> Result<(), HRESULT> {
    unsafe {
        match native::CoInitializeEx(std::ptr::null_mut(), native::COINIT::APARTMENTTHREADED) {
            HRESULT::S_OK => Ok(()),
            hr => Err(hr),
        }
    }
}

pub fn co_uninitialize() {
    unsafe {
        native::CoUninitialize();
    }
}

pub fn co_create_instance<T: ComInterface>(clsid: &CLSID) -> Result<ComRef<T>, HRESULT> {
    unsafe {
        let ptr: *mut T = null_mut();

        match native::CoCreateInstance(
            clsid,
            null_mut(),
            native::CLSCTX::INPROC_SERVER,
            &T::IID,
            transmute(&ptr),
        ) {
            HRESULT::S_OK => Ok(ComRef::new(ptr)),
            hr => Err(hr),
        }
    }
}

pub mod native {

    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    #[repr(C)]
    pub struct COINIT(pub DWORD);

    impl COINIT {
        pub const APARTMENTTHREADED: COINIT = COINIT(DWORD(0x2));
        pub const MULTITHREADED: COINIT = COINIT(DWORD(0x2));
        pub const DISABLE_OLE1DDE: COINIT = COINIT(DWORD(0x2));
        pub const SPEED_OVER_MEMORY: COINIT = COINIT(DWORD(0x2));
    }

    #[derive(Clone, Debug, PartialEq, Eq, Copy)]
    #[repr(C)]
    pub struct CLSCTX(pub DWORD);
    impl CLSCTX {
        pub const INPROC_SERVER: CLSCTX = CLSCTX(DWORD(0x1));
        pub const INPROC_HANDLER: CLSCTX = CLSCTX(DWORD(0x2));
        pub const LOCAL_SERVER: CLSCTX = CLSCTX(DWORD(0x4));
        pub const INPROC_SERVER16: CLSCTX = CLSCTX(DWORD(0x8));
        pub const REMOTE_SERVER: CLSCTX = CLSCTX(DWORD(0x10));
        pub const INPROC_HANDLER16: CLSCTX = CLSCTX(DWORD(0x20));
        pub const RESERVED1: CLSCTX = CLSCTX(DWORD(0x40));
        pub const RESERVED2: CLSCTX = CLSCTX(DWORD(0x80));
        pub const RESERVED3: CLSCTX = CLSCTX(DWORD(0x100));
        pub const RESERVED4: CLSCTX = CLSCTX(DWORD(0x200));
        pub const NO_CODE_DOWNLOAD: CLSCTX = CLSCTX(DWORD(0x400));
        pub const RESERVED5: CLSCTX = CLSCTX(DWORD(0x800));
        pub const NO_CUSTOM_MARSHAL: CLSCTX = CLSCTX(DWORD(0x1000));
        pub const ENABLE_CODE_DOWNLOAD: CLSCTX = CLSCTX(DWORD(0x2000));
        pub const NO_FAILURE_LOG: CLSCTX = CLSCTX(DWORD(0x4000));
        pub const DISABLE_AAA: CLSCTX = CLSCTX(DWORD(0x8000));
        pub const ENABLE_AAA: CLSCTX = CLSCTX(DWORD(0x10000));
        pub const FROM_DEFAULT_CONTEXT: CLSCTX = CLSCTX(DWORD(0x20000));
        pub const ACTIVATE_32_BIT_SERVER: CLSCTX = CLSCTX(DWORD(0x40000));
        pub const ACTIVATE_64_BIT_SERVER: CLSCTX = CLSCTX(DWORD(0x80000));
        pub const ENABLE_CLOAKING: CLSCTX = CLSCTX(DWORD(0x100000));
        pub const APPCONTAINER: CLSCTX = CLSCTX(DWORD(0x400000));
        pub const ACTIVATE_AAA_AS_IU: CLSCTX = CLSCTX(DWORD(0x800000));
        pub const PS_DLL: CLSCTX = CLSCTX(DWORD(0x80000000));
    }

    #[link(name = "ole32")]
    extern "stdcall" {
        pub fn CoCreateInstance(
            rclsid: &CLSID,
            pUnkOuter: *mut IUnknown,
            dwClsContext: CLSCTX,
            riid: &IID,
            ppv: *mut *mut c_void,
        ) -> HRESULT;

        pub fn CoInitializeEx(
            pvReserved: *mut c_void,
            dwCoInit: COINIT,
        ) -> HRESULT;

        pub fn CoUninitialize();
    }

}
