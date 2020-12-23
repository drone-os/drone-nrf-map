//! Nordic Semi nRFx peripheral mappings for Drone, an Embedded Operating
//! System.
//!
//! This crate uses
//! [CMSIS-SVD](https://arm-software.github.io/CMSIS_5/SVD/html/index.html)
//! files provided by [Nordic Semiconductor](https://www.nordicsemi.com/) to
//! automatically generate Drone register and interrupt bindings. However only
//! the corresponding Product Specification is the single source of truth. A
//! difference between this crate bindings and the Product Specification is
//! considered a bug. Fixing such a bug is *not a breaking change*.
//!
//! This crate re-exports the contents of [`drone_cortexm::map`] module and is a
//! drop-in replacement for it.
//!
//! # Supported Devices
//!
//! | `nrf_mcu`  | Core name              | Product specification                                                 | Available features  |
//! |------------|------------------------|-----------------------------------------------------------------------|---------------------|
//! | `nrf52810` | ARM® Cortex®-M4F r0p1  | [PS v1.3](https://infocenter.nordicsemi.com/pdf/nRF52810_PS_v1.3.pdf) |                     |
//! | `nrf52811` | ARM® Cortex®-M4F r0p1  | [PS v1.0](https://infocenter.nordicsemi.com/pdf/nRF52811_PS_v1.0.pdf) |                     |
//! | `nrf52832` | ARM® Cortex®-M4F r0p1  | [PS v1.4](https://infocenter.nordicsemi.com/pdf/nRF52832_PS_v1.4.pdf) |                     |
//! | `nrf52840` | ARM® Cortex®-M4F r0p1  | [PS v1.1](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.1.pdf) |                     |
//! | `nrf9160`  | ARM® Cortex®-M33F r0p2 | [PS v1.1](https://infocenter.nordicsemi.com/pdf/nRF9160_PS_v1.1.pdf)  | `uarte`             |
//!
//! `nrf_mcu` config flag should be set at the application level according to
//! this table.
//!
//! # Documentation
//!
//! - [Drone Book](https://book.drone-os.com/)
//! - [API documentation](https://api.drone-os.com/drone-nrf-map/0.13/)
//!
//! The API documentation intentionally skips auto-generated [`reg`] and [`thr`]
//! bindings. Otherwise it would use several gigabytes of space and would be
//! very slow to render in a browser. One should refer to the Product
//! Specification instead. And to get an idea of what the API looks like on the
//! Drone side, look at the [`drone_cortexm::map`] module documentation.
//!
//! # Usage
//!
//! Add the crate to your `Cargo.toml` dependencies:
//!
//! ```toml
//! [dependencies]
//! drone-nrf-map = { version = "0.13.0", features = [...] }
//! ```
//!
//! Add or extend `std` feature as follows:
//!
//! ```toml
//! [features]
//! std = ["drone-nrf-map/std"]
//! ```

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![no_std]

pub mod periph;
pub mod reg;
pub mod thr;

pub use drone_nrf_map_pieces::nrf_reg_tokens;
