#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

#![feature(wasm_simd)]
#![feature(wasm_target_feature)]
#![feature(target_feature_11)]

use core::mem;
use core::arch::wasm32::{self, v128};
use std::time::Instant;

const N: usize = 100_000;

fn main() {
    let now = Instant::now();
    unsafe { with_simd(); }
    let t1 = now.elapsed().as_secs_f64();
    println!("simd enabled: {}s", t1);

    let now1 = Instant::now();
    no_simd();
    let t2 = now1.elapsed().as_secs_f64();
    println!("simd disabled: {}s", t2);
    println!("simd disabled / simd enabled: {}", t2 / t1);
}

#[cfg(target_arch = "wasm32")]
#[target_feature(enable = "simd128")]
fn with_simd() {
    let a = [1i32; N];
    let b = [4i32; N];

    const step: usize = 4;
    const n: usize = N / step;
    for i in 0..n {
        let prev = i * step;
        let next = step + i * step;
        let mut _c = [0i32; 4];
        let mut _d = [0i32; 4];
        _c.copy_from_slice(&a[prev..next]);
        _d.copy_from_slice(&b[prev..next]);
        let _a: v128 = unsafe { mem::transmute(_c) };
        // let _a: v128 = wasm32::i32x4(_c[0], _c[1], _c[2], _c[3]);
        let _b: v128 = unsafe { mem::transmute(_d) };
        // let _b: v128 = wasm32::i32x4(_d[0], _d[1], _d[2], _d[3]);
        let _f = unsafe { wasm32::i32x4_add(_a, _b) };
        let _e: [i32; 4] = unsafe { mem::transmute(_f) };
        // println!("added in wasm: {:?}", _e);
    }
}

fn no_simd() {
    let a = [1i32; N];
    let b = [4i32; N];

    for i in 0..N {
        let _ = a[0] + b[0];
    }
}
