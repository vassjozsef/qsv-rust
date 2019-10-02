#![allow(non_camel_case_types, non_snake_case)]
#![feature(untagged_unions)]

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
extern crate libc;
use std::io::{Error, ErrorKind};
use std::mem;
use std::ptr;
use std::slice;

#[derive(Debug)]
struct Params {
    input: String,
    output: String,
    width: usize,
    height: usize,
    bitrate: u16,
}

pub type mfxU8 = u8;
pub type mfxU16 = u16;
pub type mfxU32 = u32;
pub type mfxI32 = i32;
pub type mfxU64 = u64;
pub type mfxI64 = i64;
pub type mfxIMPL = mfxI32;
pub type mfxStatus = mfxI32;
pub type mfxSession = libc::c_void;
pub type mfxHDL = *const libc::c_void;
pub type mfxMemId = mfxHDL;
pub type mfxSyncPoint = *const libc::c_void;

pub const MFX_IMPL_AUTO: mfxIMPL = 0x0000;
pub const MFX_IMPL_SOFTWARE: mfxIMPL = 0x0001;
pub const MFX_IMPL_HARDWARE: mfxIMPL = 0x0002;
pub const MFX_IMPL_AUTO_ANY: mfxIMPL = 0x0003;

pub const MFX_ERR_NONE: mfxStatus = 0;
pub const MFX_ERR_UNKNOWN: mfxStatus = -1;
pub const MFX_ERR_NULL_PTR: mfxStatus = -2;
pub const MFX_ERR_UNSUPPORTED: mfxStatus = -3;
pub const MFX_ERR_NOT_ENOUGH_BUFFER: mfxStatus = -5;
pub const MFX_ERR_NOT_FOUND: mfxStatus = -9;
pub const MFX_ERR_MORE_DATA: mfxStatus = -10;
pub const MFX_ERR_INVALID_VIDEO_PARAM: mfxStatus = -15;
pub const MFX_ERR_UNDEFINED_BEHAVIOR: mfxStatus = -16;

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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct mfxFrameIdStruct2 {
    pub ViewId: mfxU16,
}

impl mfxFrameIdStruct2 {
    pub fn new() -> Self {
        mfxFrameIdStruct2 { ViewId: 0 }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone)]
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
    pub In: mfxFrameInfo,
    pub Out: mfxFrameInfo,
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
pub struct mfxFrameData {
    // TODO: union ExtParam: const* const* mfxExtBuffer
    pub reserved2: mfxU64,
    pub NumExtParam: mfxU16,
    pub reserved: [mfxU16; 9],
    pub MemType: mfxU16,
    pub PitchHigh: mfxU16,
    pub TimeStamp: mfxU64,
    pub FrameOrder: mfxU32,
    pub Locked: mfxU16,
    // TODO: union Pitch
    pub PitchLow: mfxU16,

    pub Y: *mut mfxU8,
    // union
    pub UV: *mut mfxU8,
    // union
    pub V: *mut mfxU8,
    pub A: *mut mfxU8,
    pub MemId: mfxMemId,
    pub Corrupted: mfxU16,
    pub DataFlag: mfxU16,
}

impl mfxFrameData {
    pub fn new() -> Self {
        mfxFrameData {
            reserved2: 0,
            NumExtParam: 0,
            reserved: [0; 9],
            MemType: 0,
            PitchHigh: 0,
            TimeStamp: 0,
            FrameOrder: 0,
            Locked: 0,
            PitchLow: 0,

            Y: ptr::null_mut(),
            UV: ptr::null_mut(),
            V: ptr::null_mut(),
            A: ptr::null_mut(),
            MemId: ptr::null(),
            Corrupted: 0,
            DataFlag: 0,
        }
    }
}

#[repr(C)]
pub struct mfxFrameSurface1 {
    pub reserved: [mfxU32; 4],
    pub Info: mfxFrameInfo,
    pub Data: mfxFrameData,
}

impl mfxFrameSurface1 {
    pub fn new() -> Self {
        mfxFrameSurface1 {
            reserved: [0; 4],
            Info: mfxFrameInfo::new(),
            Data: mfxFrameData::new(),
        }
    }
}

#[repr(C)]
pub struct mfxBitstream {
    // TODO: union encrypted data
    pub reserved: [mfxU32; 6],
    pub DecodeTimeStamp: mfxI64,
    pub TimeStamp: mfxU64,
    pub Data: *const mfxU8,
    pub DataOffset: mfxU32,
    pub DataLength: mfxU32,
    pub MaxLength: mfxU32,
    pub PicStruct: mfxU16,
    pub FrameType: mfxU16,
    pub DataFlag: mfxU16,
    pub reserved2: mfxU16,
}

impl mfxBitstream {
    pub fn new() -> Self {
        mfxBitstream {
            reserved: [0; 6],
            DecodeTimeStamp: 0,
            TimeStamp: 0,
            Data: ptr::null(),
            DataOffset: 0,
            DataLength: 0,
            MaxLength: 0,
            PicStruct: 0,
            FrameType: 0,
            DataFlag: 0,
            reserved2: 0,
        }
    }
}

#[repr(C)]
pub struct mfxEncodeCtrl {
    pub Header: mfxExtBuffer,
    pub reserved: [mfxU32; 5],
    pub SkipFrame: mfxU16,

    pub QP: mfxU16,
    pub FrameType: mfxU16,
    pub NumExtParam: mfxU16,
    pub NumPayload: mfxU16,
    pub reserved2: mfxU16,

    pub ExtParam: *const *const mfxExtBuffer,
    pub Payload: *const *const mfxPayload,
}

#[repr(C)]
pub struct mfxPayload {
    pub reserved: [mfxU32; 4],
    pub Data: *const mfxU8,
    pub NumBit: mfxU32,
    pub Type: mfxU16,
    pub BufSize: mfxU16,
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

    pub fn MFXVideoENCODE_Init(session: *const mfxSession, par: *const mfxVideoParam) -> mfxStatus;

    pub fn MFXVideoENCODE_GetVideoParam(
        session: *const mfxSession,
        par: *mut mfxVideoParam,
    ) -> mfxStatus;

    pub fn MFXVideoENCODE_EncodeFrameAsync(
        session: *const mfxSession,
        ctrl: *const mfxEncodeCtrl,
        surface: *const mfxFrameSurface1,
        bs: *mut mfxBitstream,
        syncp: *mut mfxSyncPoint,
    ) -> mfxStatus;

    pub fn MFXVideoCORE_SyncOperation(
        session: *const mfxSession,
        syncp: mfxSyncPoint,
        wait: mfxU32,
    ) -> mfxStatus;

    pub fn MFXVideoENCODE_Close(session: *const mfxSession) -> mfxStatus;
}

fn align16(x: u16) -> u16 {
    ((x + 15) >> 4) << 4
}

fn align32(x: u32) -> u32 {
    (x + 31) & !31
}

fn GetFreeSurfaceIndex(surfaces: &Vec<mfxFrameSurface1>) -> Result<usize, mfxStatus> {
    for i in 0..surfaces.len() {
        if surfaces[i].Data.Locked == 0 {
            return Ok(i);
        }
    }

    return Err(MFX_ERR_NOT_FOUND);
}

fn ReadPlaneData(
    w: usize,
    h: usize,
    ptr: *mut mfxU8,
    pitch: usize,
    offset: usize,
    file: &mut File,
) -> Result<mfxStatus, mfxStatus> {
    let mut buf: Vec<u8> = Vec::with_capacity(w);
    buf.resize(w, 0);
    for i in 0..h {
        let rc = file.read(&mut buf);
        if rc.is_err() {
            return Err(MFX_ERR_MORE_DATA);
        }
        if rc.unwrap() != (w as usize) {
            return Err(MFX_ERR_MORE_DATA);
        }

        for j in 0..w {
            unsafe { *(ptr.offset((i * pitch + j * 2 + offset) as isize)) = buf[j] };
        }
    }

    return Ok(MFX_ERR_NONE);
}

fn LoadRawFrame(surface: &mut mfxFrameSurface1, file: &mut File) -> Result<mfxStatus, mfxStatus> {
    let pInfo = &surface.Info;
    let pData = &surface.Data;
    let w = pInfo.CropW as usize;
    let h = pInfo.CropH as usize;
    let x = pInfo.CropX as usize;
    let y = pInfo.CropY as usize;
    let pitch = pData.PitchLow as usize;

    // read luminance plane
    let ptr = unsafe { pData.Y.offset((x + y * pitch) as isize) };
    for i in 0..h {
        let slice = unsafe { slice::from_raw_parts_mut(ptr.offset((i * pitch) as isize), w) };
        let y_result = file.read(slice);
        if y_result.is_err() {
            return Err(MFX_ERR_MORE_DATA);
        }
    }

    let w_uv = w / 2;
    let h_uv = h / 2;

    let ptr_uv = unsafe { pData.UV.offset((x + y * pitch / 2) as isize) };

    // load U
    ReadPlaneData(w_uv, h_uv, ptr_uv, pitch, 0, file)?;
    // load V
    ReadPlaneData(w_uv, h_uv, ptr_uv, pitch, 1, file)?;

    return Ok(MFX_ERR_NONE);
}

fn WriteBitStreamFrame(pMfxBitstream: &mut mfxBitstream, file: &mut File) -> io::Result<()> {
    let buffer = unsafe {
        slice::from_raw_parts(
            pMfxBitstream.Data.offset(pMfxBitstream.DataOffset as isize),
            pMfxBitstream.DataLength as usize,
        )
    };
    let nBytesWritten = file.write(buffer)?;
    if nBytesWritten != (pMfxBitstream.DataLength as usize) {
        return Err(Error::from(ErrorKind::InvalidData));
    }
    pMfxBitstream.DataLength = 0;
    return Ok(());
}

fn main() -> io::Result<()> {
    println!("Size of mfxFrameInfo: {}", mem::size_of::<mfxFrameInfo>());
    println!("Size of mfxInfoMFX: {}", mem::size_of::<mfxInfoMFX>());
    println!("Size of mfxInfoVPP: {}", mem::size_of::<mfxInfoVPP>());
    println!("Size of mfxVideoParam: {}", mem::size_of::<mfxVideoParam>());

    let mut sts: mfxStatus;
    let implementation = MFX_IMPL_AUTO_ANY;
    let version = mfxVersion::new(1, 0);
    let mut session: *mut mfxSession = ptr::null_mut();
    sts = unsafe { MFXInit(implementation, &version, &mut session) };
    println!("MFX initialized: {}", sts);

    let mut actual = MFX_IMPL_AUTO_ANY;
    unsafe { MFXQueryIMPL(session, &mut actual) };
    println!("H264 implementation: 0x{:x}", actual);

    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        println!("Usage: {} input output width height bitrate", args[0]);
        return Err(Error::from(ErrorKind::InvalidInput));
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

    sts = unsafe { MFXVideoENCODE_Query(session, &mfxEncParams, &mut mfxEncParams) };
    println!("Checking encoding parameters: {}", sts);

    let mut encRequest = mfxFrameAllocRequest::new();
    sts = unsafe { MFXVideoENCODE_QueryIOSurf(session, &mfxEncParams, &mut encRequest) };
    println!("Checking surfaces: {}", sts);

    let encSurfNum: usize = encRequest.NumFrameSuggested as usize;
    let width: usize = align32(encRequest.Info.Width as u32) as usize;
    let height: usize = align32(encRequest.Info.Height as u32) as usize;
    let bitsPerPixel = 12;
    let surfaceSize = (width) * (height) * bitsPerPixel / 8;

    println!("Surfaces: {}, size: {}", encSurfNum, surfaceSize);

    let mut surfaceBuffers: Vec<u8> = Vec::with_capacity(encSurfNum * surfaceSize);
    surfaceBuffers.resize(encSurfNum * surfaceSize, 0);

    let mut pEncSurfaces: Vec<mfxFrameSurface1> = Vec::new();
    for i in 0..encSurfNum {
        let mut surface = mfxFrameSurface1::new();
        surface.Info = unsafe { mfxEncParams.u.mfx.FrameInfo.clone() };
        surface.Data.Y = unsafe {
            surfaceBuffers
                .as_mut_ptr()
                .offset((surfaceSize * i) as isize)
        };
        surface.Data.UV = unsafe { surface.Data.Y.offset((width * height) as isize) };
        surface.Data.V = unsafe { surface.Data.UV.offset(1) };
        surface.Data.PitchLow = width as u16;
        println!(
            "Surface {}, size: {} x {}",
            i, surface.Info.Width, surface.Info.Height
        );
        pEncSurfaces.push(surface);
    }

    sts = unsafe { MFXVideoENCODE_Init(session, &mfxEncParams) };
    println!("Initializing encoder: {}", sts);

    let mut par = mfxVideoParam::new();
    let getParam = unsafe { MFXVideoENCODE_GetVideoParam(session, &mut par) };
    println!("Getting encoder parameters: {}", getParam);
    let bufferSizeInKB = unsafe { par.u.mfx.BufferSizeInKB } as u32;
    println!("Buffer BufferSizeInKB: {}", bufferSizeInKB);

    let mut mfxBS = mfxBitstream::new();
    mfxBS.MaxLength = 1000 * bufferSizeInKB;
    let mut encoded: Vec<u8> = Vec::with_capacity(mfxBS.MaxLength as usize);
    encoded.resize(mfxBS.MaxLength as usize, 0);
    mfxBS.Data = encoded.as_ptr();

    let mut syncp: mfxSyncPoint = ptr::null_mut();
    let mut nFrame: mfxU32 = 0;

    let mut file_in = File::open(params.input)?;
    let mut file_out = File::create(params.output)?;

    // Stage 1: Main encoding loop
    while MFX_ERR_NONE <= sts || MFX_ERR_MORE_DATA == sts {
        let get_surface_status = GetFreeSurfaceIndex(&pEncSurfaces);
        if get_surface_status.is_err() {
            println!("Memory allocation error");
            return Err(Error::new(ErrorKind::Other, "Memory allocation error"));
        }
        let nEncSurfIdx = get_surface_status.unwrap();
        let read_status = LoadRawFrame(&mut pEncSurfaces[nEncSurfIdx], &mut file_in);
        if read_status.is_err() {
            sts = read_status.unwrap_err();
            break;
        }

        sts = unsafe {
            MFXVideoENCODE_EncodeFrameAsync(
                session,
                ptr::null(),
                &pEncSurfaces[nEncSurfIdx],
                &mut mfxBS,
                &mut syncp,
            )
        };

        println!("Encode result: {}, sync: {:#?}", sts, syncp);

        if MFX_ERR_NONE < sts {
            println!("Encode warning: {}", sts);
        }
        if MFX_ERR_NOT_ENOUGH_BUFFER == sts {
            println!("Not enough buffers");
        }
        if MFX_ERR_NONE == sts {
            sts = unsafe { MFXVideoCORE_SyncOperation(session, syncp, 6000) };
            println!("Sync resut: {}", sts);
            nFrame += 1;
            println!("Processed frame {}", nFrame);

            WriteBitStreamFrame(&mut mfxBS, &mut file_out)?;
        }
    }

    // MFX_ERR_MORE_DATA means that the input file has ended, need to go to buffering loop, exit in case of other errors
    if sts == MFX_ERR_MORE_DATA {
        sts = MFX_ERR_NONE;
    } else {
        return Err(Error::new(ErrorKind::Other, "Encode error"));
    }

    // Stage 2: Retrieve the buffered encoded frames
    while MFX_ERR_NONE <= sts {
        sts = unsafe {
            MFXVideoENCODE_EncodeFrameAsync(
                session,
                ptr::null(),
                ptr::null(),
                &mut mfxBS,
                &mut syncp,
            )
        };

        println!("Encode flush result: {}, sync: {:#?}", sts, syncp);

        if MFX_ERR_NONE < sts {
            println!("Encode flush warning: {}", sts);
        }

        if MFX_ERR_NONE == sts {
            sts = unsafe { MFXVideoCORE_SyncOperation(session, syncp, 6000) };
            println!("Sync flush resut: {}", sts);
            nFrame += 1;
            println!("Processed flush frame {}", nFrame);

            WriteBitStreamFrame(&mut mfxBS, &mut file_out)?;
        }
    }

    // MFX_ERR_MORE_DATA indicates that there are no more buffered frames, exit in case of other errors
    if sts == MFX_ERR_MORE_DATA {
        sts = MFX_ERR_NONE;
    }

    if sts != MFX_ERR_NONE {
        return Err(Error::new(ErrorKind::Other, "Encode error"));
    }

    unsafe { MFXVideoENCODE_Close(session) };

    Ok(())
}
