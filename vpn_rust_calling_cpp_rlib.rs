// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
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
}
