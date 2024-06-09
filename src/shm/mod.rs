use std::io;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::handleapi::CloseHandle;
use winapi::shared::minwindef::FALSE;
use winapi::um::memoryapi::{
    MapViewOfFile, OpenFileMappingW, UnmapViewOfFile, FILE_MAP_ALL_ACCESS, FILE_MAP_READ,
};
use serde_xml_rs::from_str;

use super::Data;
pub fn read_from_shared_memory() ->io::Result<Vec<Data>> {
    let name: Vec<u16> = OsStr::new("AIDA64_SensorValues")
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    let handle = unsafe { OpenFileMappingW(FILE_MAP_ALL_ACCESS, FALSE, name.as_ptr()) };
    if handle.is_null() {
        return  Err(io::Error::last_os_error())
    }
    let map_view = unsafe { MapViewOfFile(handle, FILE_MAP_READ, 0, 0, 0) };
    if map_view.is_null() {
        unsafe {
            CloseHandle(handle);
        }
        return Err(io::Error::last_os_error())
    }
    let mut length = 0;
    let mut d: Vec<u8> = Vec::new();
    unsafe{
        loop {
            if *(map_view as *const u8).offset(length) !=0 {
                d.push(*(map_view as *const u8).offset(length));
                length += 1;
            }else{
                UnmapViewOfFile(map_view);
                CloseHandle(handle);
                break;
            }
        }
    }
    let buf = String::from_utf8_lossy(&d);
    let data = from_str::<Vec<Data>>(&buf).unwrap();
    Ok(data)
}