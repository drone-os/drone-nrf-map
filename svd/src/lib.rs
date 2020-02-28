//! Nordic Semi nRFx SVD to bindings for Drone, an Embedded Operating System.

#![feature(str_strip)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]

pub use anyhow::{bail, Result};

use drone_svd::Device;
use std::{collections::HashSet, env, fs::File, path::Path};

const REG_EXCLUDE: &[&str] = &["FPU", "FPU_CPACR", "ITM", "MPU", "NVIC", "SCB", "STK", "TPIU"];

/// Generates code for register mappings.
pub fn generate_regs(pool_number: usize, pool_size: usize) -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize()?;
    let mut output = File::create(out_dir.join("svd_regs.rs"))?;
    drone_svd::generate_registers(&mut output, dev, pool_number, pool_size, REG_EXCLUDE)
}

/// Generates code for interrupts and register tokens struct.
pub fn generate_rest() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize()?;
    let mut reg_output = File::create(out_dir.join("svd_reg_index.rs"))?;
    let mut int_output = File::create(out_dir.join("svd_interrupts.rs"))?;
    drone_svd::generate_rest(&mut reg_output, &mut int_output, dev, "nrf_reg_tokens", REG_EXCLUDE)
}

fn svd_deserialize() -> Result<Device> {
    drone_svd::rerun_if_env_changed();
    match env::var("CARGO_CFG_NRF_MCU")?.as_ref() {
        "nrf52810" => parse_svd("nrf52810.svd"),
        "nrf52811" => parse_svd("nrf52811.svd"),
        "nrf52832" => parse_svd("nrf52.svd"),
        "nrf52840" => parse_svd("nrf52840.svd"),
        "nrf9160" => patch_nrf9160(parse_svd("nrf9160.svd")?),
        _ => bail!("invalid `nrf_mcu` cfg flag"),
    }
}

fn patch_nrf9160(mut dev: Device) -> Result<Device> {
    alias_secure_peripherals(&mut dev)?;
    Ok(dev)
}

fn alias_secure_peripherals(dev: &mut Device) -> Result<()> {
    let periph_names = dev.periph_names().cloned().collect::<HashSet<_>>();
    for periph_name in &periph_names {
        if let Some(mut base_name) = periph_name.strip_suffix("_S") {
            let periph = dev.periph(&periph_name);
            if let Some(alternate_peripheral) = &periph.alternate_peripheral {
                if let Some(alternate_base_name) = alternate_peripheral.strip_suffix("_S") {
                    base_name = alternate_base_name;
                } else {
                    continue;
                }
            }
            let alternate_peripheral = format!("{}_NS", base_name);
            if periph_names.contains(&alternate_peripheral) {
                periph.alternate_peripheral = Some(alternate_peripheral);
            }
        }
    }
    Ok(())
}

fn parse_svd(path: &str) -> Result<Device> {
    drone_svd::parse(format!("{}/files/{}", env!("CARGO_MANIFEST_DIR"), path))
}
