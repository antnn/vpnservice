// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
// https://github.com/chromium/chromium/blob/main/tools/crates/README.md
use std::env;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::unix::ffi::OsStrExt;
use std::ptr;

#[allow(unsafe_op_in_unsafe_fn)]
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn rust_calling_cpp();
    }

    unsafe extern "C++" {
        include!("vpnservice/vpn_cpp_lib.h");
        unsafe fn vpn_main_cpp(argc: i32, argv: *mut *mut c_char) -> i32;
    }
}

#[no_mangle]
pub fn rust_calling_cpp() {
    let args: Vec<CString> = env::args_os()
        .map(|os_str| {
            let bytes = os_str.as_bytes();
            CString::new(bytes).unwrap_or_else(|nul_error| {
                let nul_position = nul_error.nul_position();
                let mut bytes = nul_error.into_vec();
                bytes.truncate(nul_position);
                CString::new(bytes).unwrap()
            })
        })
        .collect();

    let argc = args.len();
    let mut argv: Vec<*mut c_char> = Vec::with_capacity(argc + 1);
    for arg in &args {
        argv.push(arg.as_ptr() as *mut c_char);
    }
    argv.push(ptr::null_mut()); // Nul terminator.

    unsafe {
        ffi::vpn_main_cpp(argc as i32, argv.as_mut_ptr());
    }

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(main_async());
}


use base64::write::EncoderStringWriter;

use std::io::{/*Read,*/ Write};
use std::net::Ipv4Addr;
use std::os::fd::AsRawFd;

use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio_tun::Tun;

use base64::engine::general_purpose;
//use base64::read::DecoderReader;

//type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


const IP_NET:&'static str ="10.1.0.0/24";



async fn main_async() {
    let queues = 3;

    let tuns = Tun::builder()
        .name("")
        .tap(false)
        .packet_info(false)
        .mtu(1500)
        .up()
        .address(Ipv4Addr::new(10, 0, 0, 1))
        .destination(Ipv4Addr::new(10, 1, 0, 1))
        .broadcast(Ipv4Addr::BROADCAST)
        .netmask(Ipv4Addr::new(255, 255, 255, 0))
        .try_build_mq(queues)
        .unwrap();

    setup_forwarding();
    disable_rp_filter();
    masquerade();

    println!("--------------");
    println!("{} tuns created", queues);
    println!("--------------");

    println!(
        "┌ name: {}\n├ fd: {}, {}, {}\n├ mtu: {}\n├ flags: {}\n├ address: {}\n├ destination: {}\n├ broadcast: {}\n└ netmask: {}",
        tuns[0].name(),
        tuns[0].as_raw_fd(), tuns[1].as_raw_fd(), tuns[2].as_raw_fd(),
        tuns[0].mtu().unwrap(),
        tuns[0].flags().unwrap(),
        tuns[0].address().unwrap(),
        tuns[0].destination().unwrap(),
        tuns[0].broadcast().unwrap(),
        tuns[0].netmask().unwrap(),
    );

    println!("---------------------");
    println!("ping 10.1.0.2 to test");
    println!("---------------------");

    let mut tuns = tuns.into_iter();
    let mut tun0 = tuns.next().unwrap();
    let mut tun1 = tuns.next().unwrap();
    let mut tun2 = tuns.next().unwrap();

    let mut buf0 = [0u8; 1500];
    let mut buf1 = [0u8; 1500];
    let mut buf2 = [0u8; 1500];

    loop {
        let (buf, _id) = tokio::select! {
            Ok(n) = tun0.read(&mut buf0) => (&buf0[..n], 0),
            Ok(n) = tun1.read(&mut buf1) => (&buf1[..n], 1),
            Ok(n) = tun2.read(&mut buf2) => (&buf2[..n], 2),
        };
        let mut enc = EncoderStringWriter::from_consumer(
            String::with_capacity(3000),
            &general_purpose::URL_SAFE,
        );

        enc.write_all(buf).unwrap();
        let b64str = enc.into_inner();
        let url = format!("http://127.0.0.1:8080/?data={}", b64str);

        print!("\n{}\n\n", url);

        //let url = url.parse().unwrap();
        //let response = fetch_json(url).await.unwrap();

        //let mut dec = DecoderReader::new(response.reader(), &general_purpose::URL_SAFE);
        //let mut data = Vec::<u8>::with_capacity(3000);

        //dec.read_to_end(&mut data).unwrap();
        //println!("decoded {:?}", data);


        let d: [u8; 84] = [
            69, 0, 0, 84, 95, 57, 64, 0, 64, 1, 207, 108, 10, 1, 0, 1, 1, 1, 1, 1, 8, 0, 254, 226,
            0, 13, 0, 36, 230, 99, 234, 100, 0, 0, 0, 0, 103, 80, 2, 0, 0, 0, 0, 0, 16, 17, 18, 19,
            20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
            42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
        ];

        if buf[9] == 1 { // Protocol Type == ICMP
            std::thread::sleep(Duration::from_millis(4000));
            println!("\nsent {:?}\n", d);
            tun0.send(&d).await.unwrap();
            std::thread::sleep(Duration::from_millis(500));
            std::process::exit(0);
        }



    }
}

use std::process::Command;
fn setup_forwarding() {

    println!("--------------");
    println!("Enable forwarding in proc",);
    println!("--------------");
    let status = Command::new("sysctl")
        .arg("net.ipv4.ip_forward=1")
        .status()
        .expect("Failed to enable forwarding");

    assert!(status.success());
}


fn disable_rp_filter() {
    println!("--------------");
    println!("Disabling rp_filter for tun0");
    println!("--------------");
    let status = Command::new("echo")
        .args(["0", ">", "/proc/sys/net/ipv4/conf/tun0/rp_filter"])
        .status()
        .expect("Failed to disable rp_filter on tun0");

    assert!(status.success());
}

fn masquerade() {
    println!("--------------");
    println!("Masquerade for {}",IP_NET);
    println!("--------------");
    //nft add rule inet firewalld nat_POSTROUTING iif tun0 masquerade
    let status = Command::new("iptables")
        .args(["-t","nat", "-A" ,"POSTROUTING", "-s", IP_NET, "-j", "MASQUERADE"])
        .status()
        .expect("Failed to Masquerade");

    assert!(status.success());
}
