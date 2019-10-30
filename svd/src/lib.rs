//! Nordic Semi nRFx SVD to bindings for Drone, an Embedded Operating System.

#![feature(generators)]
#![feature(generator_trait)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]

pub use anyhow::{bail, Result};

use drone_svd::Device;
use std::{env, fs::File, path::Path};

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

/// Generates code for register mappings.
pub fn generate_regs(pool_number: usize, pool_size: usize) -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize()?;
    let mut regs = File::create(out_dir.join("svd_regs.rs"))?;
    dev.generate_regs(&mut regs, REG_EXCLUDE, pool_number, pool_size)
}

/// Generates code for interrupts and register tokens struct.
pub fn generate_rest() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize()?;
    let mut reg_tokens = File::create(out_dir.join("svd_reg_index.rs"))?;
    let mut interrupts = File::create(out_dir.join("svd_interrupts.rs"))?;
    dev.generate_rest(
        &mut reg_tokens,
        &mut interrupts,
        REG_EXCLUDE,
        "nrf_reg_tokens",
    )
}

fn svd_deserialize() -> Result<Device> {
    drone_svd::rerun_if_env_changed();
    match env::var("CARGO_CFG_NRF_MCU")?.as_ref() {
        "nrf52810" => parse_svd("nrf52810.svd"),
        "nrf52811" => parse_svd("nrf52811.svd"),
        "nrf52832" => parse_svd("nrf52.svd"),
        "nrf52840" => parse_svd("nrf52840.svd"),
        _ => bail!("invalid `nrf_mcu` cfg flag"),
    }
}

fn parse_svd(path: &str) -> Result<Device> {
    drone_svd::parse(format!("{}/files/{}", env!("CARGO_MANIFEST_DIR"), path))
}
