#![allow(non_camel_case_types, non_snake_case)]
#![feature(untagged_unions)]

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
extern crate libc;
use std::mem;
use std::ptr;

#[derive(Debug)]
struct Params {
    input: String,
    output: String,
    width: usize,
    height: usize,
    bitrate: u16,
}

pub type mfxU16 = u16;
pub type mfxU32 = u32;
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
pub const MFX_ERR_INVALID_VIDEO_PARAM: mfxStatus = -15;

pub const MFX_WRN_INCOMPATIBLE_VIDEO_PARAM: mfxStatus = 5;

pub const MFX_TARGETUSAGE_1: u16 = 1;
pub const MFX_TARGETUSAGE_2: u16 = 2;
pub const MFX_TARGETUSAGE_3: u16 = 3;
pub const MFX_TARGETUSAGE_4: u16 = 4;
pub const MFX_TARGETUSAGE_5: u16 = 5;
pub const MFX_TARGETUSAGE_6: u16 = 6;
pub const MFX_TARGETUSAGE_7: u16 = 7;

pub const MFX_TARGETUSAGE_UNKNOWN: u16 = 0;
pub const MFX_TARGETUSAGE_BEST_QUALITY: u16 = MFX_TARGETUSAGE_1;
pub const MFX_TARGETUSAGE_BALANCED: u16 = MFX_TARGETUSAGE_4;
pub const MFX_TARGETUSAGE_BEST_SPEED: u16 = MFX_TARGETUSAGE_7;

pub const MFX_CODEC_AVC: mfxU32 = 0x20435641;
pub const MFX_FOURCC_NV12: mfxU32 = 0x3231564e;

pub const MFX_RATECONTROL_CBR: u16 = 1;
pub const MFX_RATECONTROL_VBR: u16 = 2;

pub const MFX_CHROMAFORMAT_MONOCHROME: u16 = 0;
pub const MFX_CHROMAFORMAT_YUV420: u16 = 1;

pub const MFX_PICSTRUCT_UNKNOWN: u16 = 0;
pub const MFX_PICSTRUCT_PROGRESSIVE: u16 = 1;

pub const MFX_IOPATTERN_IN_VIDEO_MEMORY: u16 = 1;
pub const MFX_IOPATTERN_IN_SYSTEM_MEMORY: u16 = 2;

#[repr(C)]
pub struct mfxVersion {
    pub Minor: mfxU16,
    pub Major: mfxU16,
}

impl mfxVersion {
    pub const fn new(Major: mfxU16, Minor: mfxU16) -> Self {
        mfxVersion { Major, Minor }
    }
}

#[repr(C)]
pub struct mfxFrameIdStruct1 {
    pub DependencyId: mfxU16,
    pub QualityId: mfxU16,
}

impl mfxFrameIdStruct1 {
    pub fn new() -> Self {
        mfxFrameIdStruct1 {
            DependencyId: 0,
            QualityId: 0,
        }
    }
}

#[repr(C)]
pub struct mfxFrameIdStruct2 {
    pub ViewId: mfxU16,
}

impl mfxFrameIdStruct2 {
    pub fn new() -> Self {
        mfxFrameIdStruct2 { ViewId: 0 }
    }
}

#[repr(C)]
pub union mfxFrameIdUnion {
    pub s1: mfxFrameIdStruct1,
    pub s2: mfxFrameIdStruct2,
}

impl mfxFrameIdUnion {
    pub fn new() -> Self {
        mfxFrameIdUnion {
            s1: mfxFrameIdStruct1::new(),
        }
    }
}

#[repr(C)]
pub struct mfxFrameId {
    pub TemporalId: mfxU16,
    pub PriorityId: mfxU16,
    pub u: mfxFrameIdUnion,
}

impl mfxFrameId {
    pub fn new() -> Self {
        mfxFrameId {
            TemporalId: 0,
            PriorityId: 0,
            u: mfxFrameIdUnion::new(),
        }
    }
}

#[repr(C)]
pub struct mfxFrameInfo {
    pub reserved: [mfxU32; 4],
    pub reserved4: mfxU16,
    pub BitDepthLuma: mfxU16,
    pub BitDepthChroma: mfxU16,
    pub Shift: mfxU16,

    pub FrameId: mfxFrameId,
    pub FourCC: mfxU32,

    // TODO: union, frame paramaters, omit buffer parameters, both 96 bits
    pub Width: mfxU16,
    pub Height: mfxU16,
    pub CropX: mfxU16,
    pub CropY: mfxU16,
    pub CropW: mfxU16,
    pub CropH: mfxU16,

    pub FrameRateExtN: mfxU32,
    pub FrameRateExtD: mfxU32,
    pub reserved3: mfxU16,

    pub AspectRatioW: mfxU16,
    pub AspectRatioH: mfxU16,

    pub PicStruct: mfxU16,
    pub ChromaFormat: mfxU16,
    pub reserved2: mfxU16,
}

impl mfxFrameInfo {
    pub fn new() -> Self {
        mfxFrameInfo {
            reserved: [0; 4],
            reserved4: 0,
            BitDepthLuma: 0,
            BitDepthChroma: 0,
            Shift: 0,
            FrameId: mfxFrameId::new(),
            FourCC: 0,
            Width: 0,
            Height: 0,
            CropX: 0,
            CropY: 0,
            CropW: 0,
            CropH: 0,
            FrameRateExtN: 0,
            FrameRateExtD: 0,
            reserved3: 0,
            AspectRatioW: 0,
            AspectRatioH: 0,
            PicStruct: 0,
            ChromaFormat: 0,
            reserved2: 0,
        }
    }
}

#[repr(C)]
pub union mfxInfoMFXUnion1 {
    pub InitialDelayInKB: mfxU16,
    pub QPP: mfxU16,
    pub Accuracy: mfxU16,
}

impl mfxInfoMFXUnion1 {
    pub fn new() -> Self {
        mfxInfoMFXUnion1 {
            InitialDelayInKB: 0,
        }
    }
}

#[repr(C)]
pub union mfxInfoMFXUnion2 {
    pub TargetKbps: mfxU16,
    pub QPP: mfxU16,
    pub ICQQuality: mfxU16,
}

impl mfxInfoMFXUnion2 {
    pub fn new() -> Self {
        mfxInfoMFXUnion2 { TargetKbps: 0 }
    }
}

#[repr(C)]
pub union mfxInfoMFXUnion3 {
    pub MaxKbps: mfxU16,
    pub QPB: mfxU16,
    pub Convergence: mfxU16,
}

impl mfxInfoMFXUnion3 {
    pub fn new() -> Self {
        mfxInfoMFXUnion3 { MaxKbps: 0 }
    }
}

#[repr(C)]
pub struct mfxInfoMFX {
    pub reserved: [mfxU32; 7],
    pub LowPower: mfxU16,
    pub BRCParamMultiplier: mfxU16,
    pub FrameInfo: mfxFrameInfo,
    pub CodecId: mfxU32,
    pub CodecProfile: mfxU16,
    pub CodecLevel: mfxU16,
    pub NumThread: mfxU16,

    //  only include encoding options
    pub TargetUsage: mfxU16,
    pub GopPicSize: mfxU16,
    pub GopRefDist: mfxU16,
    pub GopOptFlag: mfxU16,
    pub IdrInterval: mfxU16,
    pub RateControlMethod: mfxU16,
    pub u1: mfxInfoMFXUnion1,
    pub BufferSizeInKB: mfxU16,
    pub u2: mfxInfoMFXUnion2,
    pub u3: mfxInfoMFXUnion3,
    pub NumSlice: mfxU16,
    pub NumRefFrame: mfxU16,
    pub EncodedOrder: mfxU16,
}

impl mfxInfoMFX {
    pub fn new() -> Self {
        mfxInfoMFX {
            reserved: [0; 7],
            LowPower: 0,
            BRCParamMultiplier: 0,
            FrameInfo: mfxFrameInfo::new(),
            CodecId: 0,
            CodecProfile: 0,
            CodecLevel: 0,
            NumThread: 0,
            TargetUsage: 0,
            GopPicSize: 0,
            GopRefDist: 0,
            GopOptFlag: 0,
            IdrInterval: 0,
            RateControlMethod: 0,
            u1: mfxInfoMFXUnion1::new(),
            BufferSizeInKB: 0,
            u2: mfxInfoMFXUnion2::new(),
            u3: mfxInfoMFXUnion3::new(),
            NumSlice: 0,
            NumRefFrame: 0,
            EncodedOrder: 0,
        }
    }
}

#[repr(C)]
pub struct mfxExtBuffer {
    pub BufferId: mfxU32,
    pub BufferSz: mfxU32,
}

impl mfxExtBuffer {
    pub fn new() -> Self {
        mfxExtBuffer {
            BufferId: 0,
            BufferSz: 0,
        }
    }
}

#[repr(C)]
pub struct mfxInfoVPP {
    pub reserved: [mfxU32; 8],
    In: mfxFrameInfo,
    Out: mfxFrameInfo,
}

impl mfxInfoVPP {
    pub fn new() -> Self {
        mfxInfoVPP {
            reserved: [0; 8],
            In: mfxFrameInfo::new(),
            Out: mfxFrameInfo::new(),
        }
    }
}

#[repr(C)]
pub union mfxVideoParamUnion {
    pub mfx: mfxInfoMFX,
    pub vpp: mfxInfoVPP,
}

impl mfxVideoParamUnion {
    pub fn new() -> Self {
        mfxVideoParamUnion {
            mfx: mfxInfoMFX::new(),
        }
    }
}

#[repr(C)]
pub struct mfxVideoParam {
    pub AllocId: mfxU32,
    pub reserved: [mfxU32; 2],
    pub reserved3: mfxU16,
    pub AsyncDepth: mfxU16,
    pub u: mfxVideoParamUnion,
    pub Protected: mfxU16,
    pub IOPattern: mfxU16,
    pub ExtParam: *const *const mfxExtBuffer,
    pub NumExtParam: mfxU16,
    pub reserved2: mfxU16,
}

impl mfxVideoParam {
    pub fn new() -> Self {
        mfxVideoParam {
            AllocId: 0,
            reserved: [0; 2],
            reserved3: 0,
            AsyncDepth: 0,
            u: mfxVideoParamUnion::new(),
            Protected: 0,
            IOPattern: 0,
            ExtParam: ptr::null(),
            NumExtParam: 0,
            reserved2: 0,
        }
    }
}

#[repr(C)]
pub struct mfxFrameAllocRequest {
    pub AllocId: mfxU32,
    pub reserved3: [mfxU32; 3],
    pub Info: mfxFrameInfo,
    pub Type: mfxU16,
    pub NumFrameMin: mfxU16,
    pub NumFrameSuggested: mfxU16,
    pub reserved2: mfxU16,
}

impl mfxFrameAllocRequest {
    pub fn new() -> Self {
        mfxFrameAllocRequest {
            AllocId: 0,
            reserved3: [0; 3],
            Info: mfxFrameInfo::new(),
            Type: 0,
            NumFrameMin: 0,
            NumFrameSuggested: 0,
            reserved2: 0,
        }
    }
}

#[repr(C)]
pub struct mfxFrameSurface1 {
    pub reserved: [mfxU32; 4],
    pub Info: mfxFrameInfo,
    //    pub Data: mfxFrameData,
}

#[link(name = "libmfx_vs2015", kind = "static")]
extern "C" {
    pub fn MFXInit(
        implementation: mfxIMPL,
        ver: *const mfxVersion,
        session: *mut *mut mfxSession,
    ) -> mfxStatus;

    pub fn MFXQueryIMPL(session: *const mfxSession, implementation: *mut mfxIMPL) -> mfxStatus;

    pub fn MFXVideoENCODE_Query(
        session: *const mfxSession,
        input: *const mfxVideoParam,
        output: *mut mfxVideoParam,
    ) -> mfxStatus;

    pub fn MFXVideoENCODE_QueryIOSurf(
        session: *const mfxSession,
        par: *const mfxVideoParam,
        request: *mut mfxFrameAllocRequest,
    ) -> mfxStatus;
}

fn align16(x: u16) -> u16 {
    ((x + 15) >> 4) << 4
}

fn align32(x: u32) -> u32 {
    (x + 31) & !31
}

fn main() -> io::Result<()> {
    println!("Size of mfxFrameInfo: {}", mem::size_of::<mfxFrameInfo>());
    println!("Size of mfxInfoMFX: {}", mem::size_of::<mfxInfoMFX>());
    println!("Size of mfxInfoVPP: {}", mem::size_of::<mfxInfoVPP>());
    println!("Size of mfxVideoParam: {}", mem::size_of::<mfxVideoParam>());

    let implementation = MFX_IMPL_AUTO_ANY;
    let version = mfxVersion::new(1, 0);
    let mut session: *mut mfxSession = std::ptr::null_mut();
    match unsafe { MFXInit(implementation, &version, &mut session) } {
        MFX_ERR_NONE => println!("MFX initialized"),
        _ => println!("Error in MFX initialization"),
    };

    let mut actual = MFX_IMPL_AUTO_ANY;
    unsafe { MFXQueryIMPL(session, &mut actual) };
    println!("H264 implementation: 0x{:x}", actual);

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

    let mut mfxEncParams = mfxVideoParam::new();
    unsafe {
        mfxEncParams.u.mfx.CodecId = MFX_CODEC_AVC;
        mfxEncParams.u.mfx.TargetUsage = MFX_TARGETUSAGE_BALANCED;
        mfxEncParams.u.mfx.u2.TargetKbps = params.bitrate;
        mfxEncParams.u.mfx.RateControlMethod = MFX_RATECONTROL_VBR;
        mfxEncParams.u.mfx.FrameInfo.FrameRateExtN = 30;
        mfxEncParams.u.mfx.FrameInfo.FrameRateExtD = 1;
        mfxEncParams.u.mfx.FrameInfo.FourCC = MFX_FOURCC_NV12;
        mfxEncParams.u.mfx.FrameInfo.ChromaFormat = MFX_CHROMAFORMAT_YUV420;
        mfxEncParams.u.mfx.FrameInfo.PicStruct = MFX_PICSTRUCT_PROGRESSIVE;
        mfxEncParams.u.mfx.FrameInfo.CropX = 0;
        mfxEncParams.u.mfx.FrameInfo.CropY = 0;
        mfxEncParams.u.mfx.FrameInfo.CropW = params.width as u16;
        mfxEncParams.u.mfx.FrameInfo.CropH = params.height as u16;
        mfxEncParams.u.mfx.FrameInfo.Width = align16(params.width as u16);
        mfxEncParams.u.mfx.FrameInfo.Height = align16(params.height as u16);
    }
    mfxEncParams.IOPattern = MFX_IOPATTERN_IN_SYSTEM_MEMORY;

    let queryParams = unsafe { MFXVideoENCODE_Query(session, &mfxEncParams, &mut mfxEncParams) };
    println!("Checking encoding parameters: {}", queryParams);

    let mut encRequest = mfxFrameAllocRequest::new();
    let qurySurfaces =
        unsafe { MFXVideoENCODE_QueryIOSurf(session, &mfxEncParams, &mut encRequest) };
    println!("Checking surfaces: {}", qurySurfaces);

    let encSurfNum = encRequest.NumFrameSuggested;
    let width = align32(encRequest.Info.Width as u32);
    let height = align32(encRequest.Info.Height as u32);
    let bitsPerPixel = 12;
    let surfaceSize = width * height * bitsPerPixel / 8;

    println!("Surfaces: {}, size: {}", encSurfNum, surfaceSize);

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
