//! Nordic Semi nRFx peripheral mappings for Drone, an Embedded Operating
//! System.

#![feature(marker_trait_attr)]
#![feature(proc_macro_hygiene)]
#![deny(elided_lifetimes_in_paths)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(intra_doc_link_resolution_failure)]
#![no_std]

#[doc(hidden)]
pub mod reg {
    mod inner {
        pub use drone_nrf_map_pieces_1::reg::*;
        pub use drone_nrf_map_pieces_10::reg::*;
        pub use drone_nrf_map_pieces_11::reg::*;
        pub use drone_nrf_map_pieces_12::reg::*;
        pub use drone_nrf_map_pieces_2::reg::*;
        pub use drone_nrf_map_pieces_3::reg::*;
        pub use drone_nrf_map_pieces_4::reg::*;
        pub use drone_nrf_map_pieces_5::reg::*;
        pub use drone_nrf_map_pieces_6::reg::*;
        pub use drone_nrf_map_pieces_7::reg::*;
        pub use drone_nrf_map_pieces_8::reg::*;
        pub use drone_nrf_map_pieces_9::reg::*;
    }

    #[allow(unused_imports)]
    use drone_core::reg;

    include!(concat!(env!("OUT_DIR"), "/svd_reg_index.rs"));
}

#[doc(hidden)]
pub mod thr {
    mod map {
        #[allow(unused_imports)]
        use drone_cortex_m::thr;
        #[allow(unused_imports)]
        use drone_cortex_m::thr::prelude::*;

        include!(concat!(env!("OUT_DIR"), "/svd_interrupts.rs"));
    }

    pub use self::map::*;
}
