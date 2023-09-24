#!/bin/bash
# add third party
cd src
tools/rust/gnrt_stdlib.py

out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes tokio 1.28.2
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes thiserror 1.0.40
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes nix 0.26.4
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes mio 0.8.8
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes windows-targets 0.48.5
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes syn 2.0.16
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes base64 0.13.1
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes socket2 0.4.9
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes thiserror-impl 1.0.40
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes windows-sys 0.48.0
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes pin-project-lite 0.2.13
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes tokio-macros 2.1.0
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes parking_lot_core 0.9.8
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes num_cpus 1.16.0
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes lock_api 0.4.10
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes bytes 1.5.0
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes smallvec 1.11.0
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes parking_lot 0.12.1
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes signal-hook-registry 1.4.1
out/gnrt/target/release/gnrt download --security-critical=yes --shipped=yes scopeguard 1.2.0
