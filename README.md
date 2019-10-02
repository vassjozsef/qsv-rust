Quick Sync Rust Encoder
=======================

Quick Sync video encoder in Rust. Based on [https://software.intel.com/en-us/articles/media-sdk-tutorials-for-client-and-server](https://software.intel.com/en-us/articles/media-sdk-tutorials-for-client-and-server) `simple_3_encode`.

Setup
-----
Install Intel Media SDK (this sample uses 2018 R2) and set `INTELMEDIASDKROOT` (`C:\Program Files (x86)\IntelSWTools\Intel(R) Media SDK 2018 R2\Software Development Kit\`).

Active toolchain:

    nightly-2019-04-20-x86_64-pc-windows-msvc (default)
    rustc 1.36.0-nightly (8aaae4294 2019-04-19)

Build:

    cargo build

Run:

     cargo run input.yuv out.h264 1920 1080 6000