//! Nordic Semi nRFx SVD to bindings for Drone, an Embedded Operating System.

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc, clippy::unnecessary_wraps)]

pub use anyhow::{bail, Result};

use drone_svd::{Config, Device};
use std::{collections::HashSet, env, fs::File, path::Path};

/// Generates code for register mappings.
pub fn generate_regs(pool_number: usize, pool_size: usize) -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize()?;
    let mut output = File::create(out_dir.join("svd_regs.rs"))?;
    svd_config()?.generate_regs(&mut output, dev, pool_number, pool_size)
}

/// Generates code for register tokens struct.
pub fn generate_index() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);
    let dev = svd_deserialize()?;
    let mut reg_output = File::create(out_dir.join("svd_reg_index.rs"))?;
    svd_config()?.generate_index(&mut reg_output, dev)
}

fn svd_config() -> Result<Config<'static>> {
    let mut options = Config::new("nrf_reg_tokens");
    if cfg!(feature = "bit-band")
        && matches!(
            env::var("CARGO_CFG_NRF_MCU")?.as_ref(),
            "nrf52810" | "nrf52811" | "nrf52832" | "nrf52840"
        )
    {
        options.bit_band(0x4000_0000..0x4010_0000);
    }
    options.exclude_peripherals(&["FPU_NS", "FPU_S"]);
    Ok(options)
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
