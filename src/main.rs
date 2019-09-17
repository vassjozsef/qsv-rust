use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
extern crate libc;
use libc::size_t;

#[derive(Debug)]
struct Params {
    input: String,
    output: String,
    width: usize,
    height: usize,
    bitrate: u16,
}

#[link(name = "libmfx_vs2015")]
extern "C" {
    fn MFXVideoENCODE_Close(source_length: size_t) -> size_t;
}

fn main() -> io::Result<()> {
    let result = unsafe {
        MFXVideoENCODE_Close(10);
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
