#![allow(non_camel_case_types)]

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
extern crate libc;

#[derive(Debug)]
struct Params {
    input: String,
    output: String,
    width: usize,
    height: usize,
    bitrate: u16,
}

pub type mfxU16 = u16;
pub type mfxI32 = i32;
pub type mfxIMPL = mfxI32;
pub type mfxStatus = mfxI32;
pub type mfxSession = libc::c_void;

pub const MFX_IMPL_AUTO: mfxIMPL = 0x0000;
pub const MFX_IMPL_SOFTWARE: mfxIMPL = 0x0001;
pub const MFX_IMPL_HARDWARE: mfxIMPL = 0x0002;
pub const MFX_IMPL_AUTO_ANY: mfxIMPL = 0x0003;

pub const MFX_ERR_NONE: mfxStatus = 0;
pub const MFX_ERR_UNKNOWN: mfxStatus = -1;
pub const MFX_ERR_NULL_PTR: mfxStatus = -2;
pub const MFX_ERR_UNSUPPORTED: mfxStatus = -3;

#[repr(C)]
pub struct mfxVersion {
    pub minor: mfxU16,
    pub major: mfxU16,
}

#[link(name = "libmfx_vs2015", kind = "static")]
extern "C" {
    pub fn MFXInit(
        implementation: mfxIMPL,
        ver: *const mfxVersion,
        session: *mut *mut mfxSession,
    ) -> mfxStatus;
}

fn main() -> io::Result<()> {
    let implementation = MFX_IMPL_AUTO_ANY;
    let version: mfxVersion = mfxVersion { minor: 0, major: 1 };
    let mut session: *mut mfxSession = std::ptr::null_mut();
    match unsafe { MFXInit(implementation, &version, &mut session) } {
        MFX_ERR_NONE => println!("MFX initialized"),
        _ => println!("Error in MFX initialization"),
    };

    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        println!("Usage: {} input output width height bitrate", args[0]);
        std::process::exit(-1);
    }
    let params = Params {
        input: args[1].clone(),
        output: args[2].clone(),
        width: args[3].parse::<usize>().unwrap(),
        height: args[4].parse::<usize>().unwrap(),
        bitrate: args[5].parse::<u16>().unwrap(),
    };
    println!("{:?}", params);

    let mut file = File::open(params.input)?;
    let frame_size: usize = params.width * params.height * 3 / 2;
    let mut v: Vec<u8> = Vec::with_capacity(frame_size);
    unsafe {
        v.set_len(frame_size);
    }
    let mut frame = 0;
    loop {
        let bytes = file.read(&mut v)?;
        if bytes == 0 {
            break;
        }
        frame += 1;
        println!("Read frame {} of {} bytes", frame, bytes);
    }
    Ok(())
}
