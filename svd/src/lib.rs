//! Nordic Semi nRFx SVD to bindings for Drone, an Embedded Operating System.

#![feature(generators)]
#![feature(generator_trait)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]

use drone_svd::Device;
use failure::Error;
use std::{env, fs::File, path::Path, process};

const REG_EXCLUDE: &[&str] = &[
    "FPU",
    "FPU_CPACR",
    "ITM",
    "MPU",
    "NVIC",
    "SCB",
    "STK",
    "TPIU",
];

/// Returns the selected device feature.
#[macro_export]
macro_rules! svd_feature {
    () => {
        if cfg!(feature = "nrf52810") {
            "nrf52810"
        } else if cfg!(feature = "nrf52811") {
            "nrf52811"
        } else if cfg!(feature = "nrf52832") {
            "nrf52832"
        } else if cfg!(feature = "nrf52840") {
            "nrf52840"
        } else {
            ""
        }
    };
}

/// Generates code for register mappings.
pub fn generate_regs(feature: &str, pool_number: usize, pool_size: usize) {
    let run = || {
        let out_dir = env::var("OUT_DIR")?;
        let out_dir = Path::new(&out_dir);
        let dev = svd_deserialize(feature)?;
        let mut regs = File::create(out_dir.join("svd_regs.rs"))?;
        dev.generate_regs(&mut regs, REG_EXCLUDE, pool_number, pool_size)?;
        Ok::<(), Error>(())
    };
    if let Err(error) = run() {
        eprintln!("{}", error);
        process::exit(1);
    }
}

/// Generates code for interrupts and register tokens struct.
pub fn generate_rest(feature: &str) {
    let run = || {
        let out_dir = env::var("OUT_DIR")?;
        let out_dir = Path::new(&out_dir);
        let dev = svd_deserialize(feature)?;
        let mut reg_tokens = File::create(out_dir.join("svd_reg_index.rs"))?;
        let mut interrupts = File::create(out_dir.join("svd_interrupts.rs"))?;
        dev.generate_rest(
            &mut reg_tokens,
            &mut interrupts,
            REG_EXCLUDE,
            "nrf_reg_tokens",
        )?;
        Ok::<(), Error>(())
    };
    if let Err(error) = run() {
        eprintln!("{}", error);
        process::exit(1);
    }
}

fn svd_deserialize(feature: &str) -> Result<Device, Error> {
    match feature {
        "nrf52810" => parse_svd("nrf52810.svd"),
        "nrf52811" => parse_svd("nrf52811.svd"),
        "nrf52832" => parse_svd("nrf52.svd"),
        "nrf52840" => parse_svd("nrf52840.svd"),
        _ => Ok(Device::new("Generic nRF".to_string())),
    }
}

fn parse_svd(path: &str) -> Result<Device, Error> {
    drone_svd::parse(format!("{}/files/{}", env!("CARGO_MANIFEST_DIR"), path))
}
