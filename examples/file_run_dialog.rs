use idl::{com_interface};
use safercom::{
    ole32::{
        ComServer,
    },
    types::{
        CLSID,
        HRESULT
    },
    ComInterface,
    ComClass,
    IDispatch
};
use std::{
    ffi::c_void,
    mem::transmute,
};

fn main() -> Result<(), HRESULT> {

    let com = ComServer::initialize()?;

    let shell_app = com.create_instance::<Shell>()?;

    shell_app.file_run()?;

    Ok(())
}

#[allow(non_camel_case_types)]
pub struct Shell;

impl ComClass for Shell {
    const CLSID: CLSID = CLSID::new(
        0x13709620,
        0xC279,
        0x11CE,
        [0xA4, 0x9E, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00],
    );

    type ClassInterface = IShellDispatch;
}

#[com_interface(iid = "0xd8f015c0-c278-11ce-a49e-444553540000")]
#[repr(C)]
pub struct IShellDispatch;

#[repr(C)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct IShellDispatch_VTable {
    pub __IDispatch:      <IDispatch as ComInterface>::VTable,
    pub Application:      extern "stdcall" fn(*const IShellDispatch, *mut *mut IDispatch) -> HRESULT,
    pub Parent:           extern "stdcall" fn(*const IShellDispatch, *mut *mut IDispatch) -> HRESULT,
    pub NameSpace:        *mut c_void, // extern "stdcall" fn(*const IShellDispatch, VARIANT, *mut *mut Folder) -> HRESULT,
    pub BrowseForFolder:  *mut c_void, // extern "stdcall" fn(*const IShellDispatch, u32, BSTR, u32, *mut *mut Folder) -> HRESULT,
    pub Windows:          extern "stdcall" fn(*const IShellDispatch, *mut *mut IDispatch) -> HRESULT,
    pub Open:             *mut c_void, // extern "stdcall" fn(*const IShellDispatch, VARIANT) -> HRESULT,
    pub Explore:          *mut c_void, // extern "stdcall" fn(*const IShellDispatch, VARIANT) -> HRESULT,
    pub MinimizeAll:      extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub UndoMinimizeALL:  extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub FileRun:          extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub CascadeWindows:   extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub TileVertically:   extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub TileHorizontally: extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub ShutdownWindows:  extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub Suspend:          extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub EjectPC:          extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub SetTime:          extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub TrayProperties:   extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub Help:             extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub FindFiles:        extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub FindComputer:     extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub RefreshMenu:      extern "stdcall" fn(*const IShellDispatch) -> HRESULT,
    pub ControlPanelItem: *mut c_void, // extern "stdcall" fn(*const IShellDispatch, BSTR) -> HRESULT,
}

impl IShellDispatch {
    pub fn file_run(&self) -> Result<(), HRESULT> {
        unsafe {
            match ((*self.vtable).FileRun)(transmute(self)) {
                HRESULT::S_OK => Ok(()),
                hr => Err(hr),
            }
        }
    }
}