use oot_display::SaveData;
use std::env::args;

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let pid: u32 = args[0].parse().unwrap();
    let reader = MemReader::new(pid).unwrap();
    let addr = 0x6409_4B42;
    let bytes = reader
        .read_bytes(addr, std::mem::size_of::<SaveData>())
        .unwrap();
    let mut save_data = unsafe { std::ptr::read(bytes.as_ptr() as *const SaveData) };
    println!("{:#?}", save_data);
    save_data.item_slot_item_ids.bomb = oot_display::ItemID::Bomb;
    save_data.item_slot_amount.bomb = 7;
    let _res = unsafe {
        kernel32::WriteProcessMemory(
            reader.handle.load(Ordering::Relaxed),
            addr as *mut _,
            &mut save_data as *mut _ as *mut _,
            std::mem::size_of::<SaveData>() as u64,
            std::ptr::null_mut(),
        )
    };
    /*
    let handle =
        unsafe { kernel32::OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, 0, pid) };
    if handle.is_null() {
        panic!("handle is null");
    }
    // kernel32::GetModuleHandleA(b"mupen64plus.dll\0" as *const u8 as *const i8);
    let mut buffer = [0u8; 4];
    println!("{}", unsafe {
        kernel32::ReadProcessMemory(
            handle as *mut c_void,
            addr as *const c_void,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len() as u64,
            std::ptr::null_mut(),
        )
    });
    println!("{:?}", buffer);
    */
}
/*

use memreader::{MemReader, ProvidesSlices};

use std::io::Read;

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let pid: u32 = args[0].parse().unwrap();
    let addr: usize = args[1].parse().unwrap();
    let n: usize = args[2].parse().unwrap();

    let reader = MemReader::new(pid).unwrap();

    let mut buf = [0];

    let page_size: i64 = 4096;
    let max_mem: i64 = 2 * 1024 * 1024 * 1024;

    for i in 0.. {
        if reader
            .address_slice_len(i as usize, 1)
            .read_exact(&mut buf)
            .is_ok()
            && buf == target
        {
            print!("{:02X} | ", i);
            for &x in &buf {
                print!("{:02X}", x);
            }
            println!();
        }
    }
}

*/

use kernel32::{K32EnumProcessModulesEx, K32GetModuleBaseNameA, OpenProcess, ReadProcessMemory};
use winapi::minwindef::{DWORD, HMODULE, MAX_PATH};
use winapi::psapi::{LIST_MODULES_32BIT, LIST_MODULES_64BIT};
use winapi::winnt::{
    PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION, PROCESS_VM_READ, PROCESS_VM_WRITE,
};

use std::ffi::CStr;
use std::mem::{size_of, size_of_val, uninitialized};
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct MemReader {
    handle: AtomicPtr<::std::os::raw::c_void>,
}

impl MemReader {
    pub fn new(pid: u32) -> Option<MemReader> {
        let handle = unsafe {
            OpenProcess(
                PROCESS_VM_READ
                    | PROCESS_QUERY_INFORMATION
                    | PROCESS_VM_WRITE
                    | PROCESS_VM_OPERATION,
                0,
                pid,
            )
        };
        if handle.is_null() {
            None
        } else {
            Some(MemReader {
                handle: AtomicPtr::new(handle),
            })
        }
    }

    pub fn base_address(&self, process_name: &str) -> Option<usize> {
        let mut hmod: HMODULE = unsafe { uninitialized() };
        let mut cb_needed: DWORD = unsafe { uninitialized() };
        let res = unsafe {
            K32EnumProcessModulesEx(
                self.handle.load(Ordering::Relaxed),
                &mut hmod as *mut HMODULE,
                size_of_val(&hmod) as u32,
                &mut cb_needed as *mut DWORD,
                LIST_MODULES_32BIT | LIST_MODULES_64BIT,
            )
        };
        if res == 0 {
            return None;
        }
        let mut base_name: [::std::os::raw::c_char; MAX_PATH] = [0; MAX_PATH];
        unsafe {
            K32GetModuleBaseNameA(
                self.handle.load(Ordering::Relaxed),
                hmod,
                &mut base_name[0] as *mut _,
                base_name.len() as u32 / size_of::<::std::os::raw::c_char>() as u32,
            );
        }
        let base_name = unsafe { CStr::from_ptr(&base_name[0] as *const _) };
        let base_name = match base_name.to_str() {
            Ok(n) => n,
            Err(_) => return None,
        };
        if base_name.to_lowercase() == process_name.to_lowercase() {
            Some(hmod as usize)
        } else {
            None
        }
    }

    fn read_bytes(&self, address: usize, n: usize) -> Option<Vec<u8>> {
        let mut buffer: Vec<u8> = vec![0; n];
        let mut read: u64 = unsafe { ::std::mem::uninitialized() };
        let res = unsafe {
            ReadProcessMemory(
                self.handle.load(Ordering::Relaxed),
                address as *const _,
                buffer.as_mut_ptr() as *mut _,
                n as u64,
                &mut read as *mut _,
            )
        };
        if res != 1 {
            return None;
        }
        if read != n as u64 {
            return None;
        }
        Some(buffer)
    }
}
