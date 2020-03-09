//! nRFx peripheral mappings.

#[doc(no_inline)]
pub use drone_cortex_m::map::periph::*;

#[cfg(feature = "uarte")]
pub extern crate drone_nrf_map_periph_uarte as uarte;
