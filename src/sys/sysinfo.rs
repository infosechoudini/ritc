use core::marker::Copy;
use core::option::Option::Some;
use core::option::Option;
use core::result::Result::Err;
use core::result::Result::Ok;
use core::marker::Sync;
use core::marker::Send;
use core::option::Option::None;
use core::mem::MaybeUninit;
use core::prelude::rust_2024;


use crate::nr;
use crate::arch::syscall0;

/// System info structure returned by `sysinfo`.
#[rust_2024::derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct SysInfo{
    pub uptime: i64,
    pub loads: [u64; 3],
    pub totalram: u64,
    pub freeram: u64,
    pub sharedram: u64,
    pub bufferram: u64,
    pub totalswap: u64,
    pub procs: u16,
    pub pad: u16,
    pub totalhigh: u64,
    pub freehigh: u64,
    pub mem_unit: u32,
    _f: [ i8 ; 0 ],
}

pub fn sysinfo() -> Option<SysInfo> {

    unsafe { 

        let mut buf = MaybeUninit::uninit();
        let ret = syscall0(nr::SYSINFO);
        buf.write(ret );

        let value =  buf.assume_init() as *const SysInfo;
        return Some(*value);

    }

    /* 
    let mut info = core::mem::MaybeUninit::uninit();
    let res = unsafe { syscall0(nr::SYSINFO) };
    info.write(res as *mut SysInfo);
    unsafe { 
        let ret = info.assume_init();
        return Some(*ret);
    }
    */

}