//! Universal Asynchronous Receiver/Transmitter.

#![feature(proc_macro_hygiene)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::type_repetition_in_bounds, clippy::wildcard_imports)]
#![no_std]

use drone_core::periph;
use drone_cortexm::reg::marker::*;

periph! {
    /// Generic UARTE peripheral variant.
    pub trait UarteMap {}

    /// Generic UARTE peripheral.
    pub struct UartePeriph;

    UARTE {
        TASKS_STARTRX {
            0x20 WoReg;
            TASKS_STARTRX { WoWoRegFieldBit }
        }
        TASKS_STOPRX {
            0x20 WoReg;
            TASKS_STOPRX { WoWoRegFieldBit }
        }
        TASKS_STARTTX {
            0x20 WoReg;
            TASKS_STARTTX { WoWoRegFieldBit }
        }
        TASKS_STOPTX {
            0x20 WoReg;
            TASKS_STOPTX { WoWoRegFieldBit }
        }
        TASKS_FLUSHRX {
            0x20 WoReg;
            TASKS_FLUSHRX { WoWoRegFieldBit }
        }
        SUBSCRIBE_STARTRX {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        SUBSCRIBE_STOPRX {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        SUBSCRIBE_STARTTX {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        SUBSCRIBE_STOPTX {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        SUBSCRIBE_FLUSHRX {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        EVENTS_CTS {
            0x20 RwReg;
            EVENTS_CTS { RwRwRegFieldBit }
        }
        EVENTS_NCTS {
            0x20 RwReg;
            EVENTS_NCTS { RwRwRegFieldBit }
        }
        EVENTS_RXDRDY {
            0x20 RwReg;
            EVENTS_RXDRDY { RwRwRegFieldBit }
        }
        EVENTS_ENDRX {
            0x20 RwReg;
            EVENTS_ENDRX { RwRwRegFieldBit }
        }
        EVENTS_TXDRDY {
            0x20 RwReg;
            EVENTS_TXDRDY { RwRwRegFieldBit }
        }
        EVENTS_ENDTX {
            0x20 RwReg;
            EVENTS_ENDTX { RwRwRegFieldBit }
        }
        EVENTS_ERROR {
            0x20 RwReg;
            EVENTS_ERROR { RwRwRegFieldBit }
        }
        EVENTS_RXTO {
            0x20 RwReg;
            EVENTS_RXTO { RwRwRegFieldBit }
        }
        EVENTS_RXSTARTED {
            0x20 RwReg;
            EVENTS_RXSTARTED { RwRwRegFieldBit }
        }
        EVENTS_TXSTARTED {
            0x20 RwReg;
            EVENTS_TXSTARTED { RwRwRegFieldBit }
        }
        EVENTS_TXSTOPPED {
            0x20 RwReg;
            EVENTS_TXSTOPPED { RwRwRegFieldBit }
        }
        PUBLISH_CTS {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        PUBLISH_NCTS {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        PUBLISH_RXDRDY {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        PUBLISH_ENDRX {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        PUBLISH_TXDRDY {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        PUBLISH_ENDTX {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        PUBLISH_ERROR {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        PUBLISH_RXTO {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        PUBLISH_RXSTARTED {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        PUBLISH_TXSTARTED {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        PUBLISH_TXSTOPPED {
            0x20 RwReg;
            CHIDX { RwRwRegFieldBits }
            EN { RwRwRegFieldBit }
        }
        SHORTS {
            0x20 RwReg;
            ENDRX_STARTRX { RwRwRegFieldBit }
            ENDRX_STOPRX { RwRwRegFieldBit }
        }
        INTEN {
            0x20 RwReg;
            CTS { RwRwRegFieldBit }
            NCTS { RwRwRegFieldBit }
            RXDRDY { RwRwRegFieldBit }
            ENDRX { RwRwRegFieldBit }
            TXDRDY { RwRwRegFieldBit }
            ENDTX { RwRwRegFieldBit }
            ERROR { RwRwRegFieldBit }
            RXTO { RwRwRegFieldBit }
            RXSTARTED { RwRwRegFieldBit }
            TXSTARTED { RwRwRegFieldBit }
            TXSTOPPED { RwRwRegFieldBit }
        }
        INTENSET {
            0x20 RwReg;
            CTS { RwRwRegFieldBit }
            NCTS { RwRwRegFieldBit }
            RXDRDY { RwRwRegFieldBit }
            ENDRX { RwRwRegFieldBit }
            TXDRDY { RwRwRegFieldBit }
            ENDTX { RwRwRegFieldBit }
            ERROR { RwRwRegFieldBit }
            RXTO { RwRwRegFieldBit }
            RXSTARTED { RwRwRegFieldBit }
            TXSTARTED { RwRwRegFieldBit }
            TXSTOPPED { RwRwRegFieldBit }
        }
        INTENCLR {
            0x20 RwReg;
            CTS { RwRwRegFieldBit }
            NCTS { RwRwRegFieldBit }
            RXDRDY { RwRwRegFieldBit }
            ENDRX { RwRwRegFieldBit }
            TXDRDY { RwRwRegFieldBit }
            ENDTX { RwRwRegFieldBit }
            ERROR { RwRwRegFieldBit }
            RXTO { RwRwRegFieldBit }
            RXSTARTED { RwRwRegFieldBit }
            TXSTARTED { RwRwRegFieldBit }
            TXSTOPPED { RwRwRegFieldBit }
        }
        ERRORSRC {
            0x20 RwReg;
            OVERRUN { RwRwRegFieldBit }
            PARITY { RwRwRegFieldBit }
            FRAMING { RwRwRegFieldBit }
            BREAK { RwRwRegFieldBit }
        }
        ENABLE {
            0x20 RwReg;
            ENABLE { RwRwRegFieldBits }
        }
        PSEL_RTS {
            0x20 RwReg;
            PIN { RwRwRegFieldBits }
            CONNECT { RwRwRegFieldBit }
        }
        PSEL_TXD {
            0x20 RwReg;
            PIN { RwRwRegFieldBits }
            CONNECT { RwRwRegFieldBit }
        }
        PSEL_CTS {
            0x20 RwReg;
            PIN { RwRwRegFieldBits }
            CONNECT { RwRwRegFieldBit }
        }
        PSEL_RXD {
            0x20 RwReg;
            PIN { RwRwRegFieldBits }
            CONNECT { RwRwRegFieldBit }
        }
        BAUDRATE {
            0x20 RwReg;
            BAUDRATE { RwRwRegFieldBits }
        }
        RXD_PTR {
            0x20 RwReg;
            PTR { RwRwRegFieldBits }
        }
        RXD_MAXCNT {
            0x20 RwReg;
            MAXCNT { RwRwRegFieldBits }
        }
        RXD_AMOUNT {
            0x20 RoReg;
            AMOUNT { RoRoRegFieldBits }
        }
        TXD_PTR {
            0x20 RwReg;
            PTR { RwRwRegFieldBits }
        }
        TXD_MAXCNT {
            0x20 RwReg;
            MAXCNT { RwRwRegFieldBits }
        }
        TXD_AMOUNT {
            0x20 RoReg;
            AMOUNT { RoRoRegFieldBits }
        }
        CONFIG {
            0x20 RwReg;
            HWFC { RwRwRegFieldBit }
            PARITY { RwRwRegFieldBits }
            STOP { RwRwRegFieldBit }
        }
    }
}

#[allow(unused_macros)]
macro_rules! map_uarte_ns_s {
    (
        $uarte_ns_macro_doc:expr,
        $uarte_s_macro_doc:expr,
        $uarte_ns_macro:ident,
        $uarte_s_macro:ident,
        $uarte_ns_ty_doc:expr,
        $uarte_s_ty_doc:expr,
        $uarte_ns_ty:ident,
        $uarte_s_ty:ident,
        $uarte_ns:ident,
        $uarte_s:ident,
        $spim_ns:ident,
        $spis_ns:ident,
        $twim_ns:ident,
    ) => {
        map_uarte!(
            $uarte_ns_macro_doc,
            $uarte_ns_macro,
            $uarte_ns_ty_doc,
            $uarte_ns_ty,
            $uarte_ns,
            (),
            $spim_ns,
            $spis_ns,
            $twim_ns,
        );
        map_uarte!(
            $uarte_s_macro_doc,
            $uarte_s_macro,
            $uarte_s_ty_doc,
            $uarte_s_ty,
            $uarte_s,
            ($uarte_ns),
            $spim_ns,
            $spis_ns,
            $twim_ns,
        );
    };
}

#[allow(unused_macros)]
macro_rules! map_uarte {
    (
        $uarte_macro_doc:expr,
        $uarte_macro:ident,
        $uarte_ty_doc:expr,
        $uarte_ty:ident,
        $uarte:ident,
        ($($uarte_ns:ident)?),
        $spim_ns:ident,
        $spis_ns:ident,
        $twim_ns:ident,
    ) => {
        periph::map! {
            #[doc = $uarte_macro_doc]
            pub macro $uarte_macro;

            #[doc = $uarte_ty_doc]
            pub struct $uarte_ty;

            impl UarteMap for $uarte_ty {}

            drone_nrf_map_pieces::reg;
            crate;

            UARTE {
                $uarte;
                TASKS_STARTRX {
                    TASKS_STARTRX($twim_ns TASKS_STARTRX);
                    TASKS_STARTRX { TASKS_STARTRX }
                }
                TASKS_STOPRX {
                    TASKS_STOPRX$(($uarte_ns TASKS_STOPRX))*;
                    TASKS_STOPRX { TASKS_STOPRX }
                }
                TASKS_STARTTX {
                    TASKS_STARTTX($twim_ns TASKS_STARTTX);
                    TASKS_STARTTX { TASKS_STARTTX }
                }
                TASKS_STOPTX {
                    TASKS_STOPTX$(($uarte_ns TASKS_STOPTX))*;
                    TASKS_STOPTX { TASKS_STOPTX }
                }
                TASKS_FLUSHRX {
                    TASKS_FLUSHRX$(($uarte_ns TASKS_FLUSHRX))*;
                    TASKS_FLUSHRX { TASKS_FLUSHRX }
                }
                SUBSCRIBE_STARTRX {
                    SUBSCRIBE_STARTRX($twim_ns SUBSCRIBE_STARTRX);
                    CHIDX { CHIDX }
                    EN { EN }
                }
                SUBSCRIBE_STOPRX {
                    SUBSCRIBE_STOPRX$(($uarte_ns SUBSCRIBE_STOPRX))*;
                    CHIDX { CHIDX }
                    EN { EN }
                }
                SUBSCRIBE_STARTTX {
                    SUBSCRIBE_STARTTX($twim_ns SUBSCRIBE_STARTTX);
                    CHIDX { CHIDX }
                    EN { EN }
                }
                SUBSCRIBE_STOPTX {
                    SUBSCRIBE_STOPTX$(($uarte_ns SUBSCRIBE_STOPTX))*;
                    CHIDX { CHIDX }
                    EN { EN }
                }
                SUBSCRIBE_FLUSHRX {
                    SUBSCRIBE_FLUSHRX$(($uarte_ns SUBSCRIBE_FLUSHRX))*;
                    CHIDX { CHIDX }
                    EN { EN }
                }
                EVENTS_CTS {
                    EVENTS_CTS$(($uarte_ns EVENTS_CTS))*;
                    EVENTS_CTS { EVENTS_CTS }
                }
                EVENTS_NCTS {
                    EVENTS_NCTS($spim_ns EVENTS_STOPPED);
                    EVENTS_NCTS { EVENTS_NCTS }
                }
                EVENTS_RXDRDY {
                    EVENTS_RXDRDY$(($uarte_ns EVENTS_RXDRDY))*;
                    EVENTS_RXDRDY { EVENTS_RXDRDY }
                }
                EVENTS_ENDRX {
                    EVENTS_ENDRX($spim_ns EVENTS_ENDRX);
                    EVENTS_ENDRX { EVENTS_ENDRX }
                }
                EVENTS_TXDRDY {
                    EVENTS_TXDRDY$(($uarte_ns EVENTS_TXDRDY))*;
                    EVENTS_TXDRDY { EVENTS_TXDRDY }
                }
                EVENTS_ENDTX {
                    EVENTS_ENDTX($spim_ns EVENTS_ENDTX);
                    EVENTS_ENDTX { EVENTS_ENDTX }
                }
                EVENTS_ERROR {
                    EVENTS_ERROR($twim_ns EVENTS_ERROR);
                    EVENTS_ERROR { EVENTS_ERROR }
                }
                EVENTS_RXTO {
                    EVENTS_RXTO$(($uarte_ns EVENTS_RXTO))*;
                    EVENTS_RXTO { EVENTS_RXTO }
                }
                EVENTS_RXSTARTED {
                    EVENTS_RXSTARTED($spim_ns EVENTS_STARTED);
                    EVENTS_RXSTARTED { EVENTS_RXSTARTED }
                }
                EVENTS_TXSTARTED {
                    EVENTS_TXSTARTED($twim_ns EVENTS_TXSTARTED);
                    EVENTS_TXSTARTED { EVENTS_TXSTARTED }
                }
                EVENTS_TXSTOPPED {
                    EVENTS_TXSTOPPED$(($uarte_ns EVENTS_TXSTOPPED))*;
                    EVENTS_TXSTOPPED { EVENTS_TXSTOPPED }
                }
                PUBLISH_CTS {
                    PUBLISH_CTS$(($uarte_ns PUBLISH_CTS))*;
                    CHIDX { CHIDX }
                    EN { EN }
                }
                PUBLISH_NCTS {
                    PUBLISH_NCTS($spim_ns PUBLISH_STOPPED);
                    CHIDX { CHIDX }
                    EN { EN }
                }
                PUBLISH_RXDRDY {
                    PUBLISH_RXDRDY$(($uarte_ns PUBLISH_RXDRDY))*;
                    CHIDX { CHIDX }
                    EN { EN }
                }
                PUBLISH_ENDRX {
                    PUBLISH_ENDRX($spim_ns PUBLISH_ENDRX);
                    CHIDX { CHIDX }
                    EN { EN }
                }
                PUBLISH_TXDRDY {
                    PUBLISH_TXDRDY$(($uarte_ns PUBLISH_TXDRDY))*;
                    CHIDX { CHIDX }
                    EN { EN }
                }
                PUBLISH_ENDTX {
                    PUBLISH_ENDTX($spim_ns PUBLISH_ENDTX);
                    CHIDX { CHIDX }
                    EN { EN }
                }
                PUBLISH_ERROR {
                    PUBLISH_ERROR($twim_ns PUBLISH_ERROR);
                    CHIDX { CHIDX }
                    EN { EN }
                }
                PUBLISH_RXTO {
                    PUBLISH_RXTO$(($uarte_ns PUBLISH_RXTO))*;
                    CHIDX { CHIDX }
                    EN { EN }
                }
                PUBLISH_RXSTARTED {
                    PUBLISH_RXSTARTED($spim_ns PUBLISH_STARTED);
                    CHIDX { CHIDX }
                    EN { EN }
                }
                PUBLISH_TXSTARTED {
                    PUBLISH_TXSTARTED($twim_ns PUBLISH_TXSTARTED);
                    CHIDX { CHIDX }
                    EN { EN }
                }
                PUBLISH_TXSTOPPED {
                    PUBLISH_TXSTOPPED$(($uarte_ns PUBLISH_TXSTOPPED))*;
                    CHIDX { CHIDX }
                    EN { EN }
                }
                SHORTS {
                    SHORTS($spim_ns SHORTS);
                    ENDRX_STARTRX { ENDRX_STARTRX }
                    ENDRX_STOPRX { ENDRX_STOPRX }
                }
                INTEN {
                    INTEN($twim_ns INTEN);
                    CTS { CTS }
                    NCTS { NCTS }
                    RXDRDY { RXDRDY }
                    ENDRX { ENDRX }
                    TXDRDY { TXDRDY }
                    ENDTX { ENDTX }
                    ERROR { ERROR }
                    RXTO { RXTO }
                    RXSTARTED { RXSTARTED }
                    TXSTARTED { TXSTARTED }
                    TXSTOPPED { TXSTOPPED }
                }
                INTENSET {
                    INTENSET($spim_ns INTENSET);
                    CTS { CTS }
                    NCTS { NCTS }
                    RXDRDY { RXDRDY }
                    ENDRX { ENDRX }
                    TXDRDY { TXDRDY }
                    ENDTX { ENDTX }
                    ERROR { ERROR }
                    RXTO { RXTO }
                    RXSTARTED { RXSTARTED }
                    TXSTARTED { TXSTARTED }
                    TXSTOPPED { TXSTOPPED }
                }
                INTENCLR {
                    INTENCLR($spim_ns INTENCLR);
                    CTS { CTS }
                    NCTS { NCTS }
                    RXDRDY { RXDRDY }
                    ENDRX { ENDRX }
                    TXDRDY { TXDRDY }
                    ENDTX { ENDTX }
                    ERROR { ERROR }
                    RXTO { RXTO }
                    RXSTARTED { RXSTARTED }
                    TXSTARTED { TXSTARTED }
                    TXSTOPPED { TXSTOPPED }
                }
                ERRORSRC {
                    ERRORSRC$(($uarte_ns ERRORSRC))*;
                    OVERRUN { OVERRUN }
                    PARITY { PARITY }
                    FRAMING { FRAMING }
                    BREAK { BREAK }
                }
                ENABLE {
                    ENABLE($spim_ns ENABLE);
                    ENABLE { ENABLE }
                }
                PSEL_RTS {
                    PSEL_RTS($spim_ns PSEL_SCK);
                    PIN { PIN }
                    CONNECT { CONNECT }
                }
                PSEL_TXD {
                    PSEL_TXD($spim_ns PSEL_MOSI);
                    PIN { PIN }
                    CONNECT { CONNECT }
                }
                PSEL_CTS {
                    PSEL_CTS($spim_ns PSEL_MISO);
                    PIN { PIN }
                    CONNECT { CONNECT }
                }
                PSEL_RXD {
                    PSEL_RXD($spis_ns PSEL_CSN);
                    PIN { PIN }
                    CONNECT { CONNECT }
                }
                BAUDRATE {
                    BAUDRATE($spim_ns FREQUENCY);
                    BAUDRATE { BAUDRATE }
                }
                RXD_PTR {
                    RXD_PTR($spim_ns RXD_PTR);
                    PTR { PTR }
                }
                RXD_MAXCNT {
                    RXD_MAXCNT($spim_ns RXD_MAXCNT);
                    MAXCNT { MAXCNT }
                }
                RXD_AMOUNT {
                    RXD_AMOUNT($spim_ns RXD_AMOUNT);
                    AMOUNT { AMOUNT }
                }
                TXD_PTR {
                    TXD_PTR($spim_ns TXD_PTR);
                    PTR { PTR }
                }
                TXD_MAXCNT {
                    TXD_MAXCNT($spim_ns TXD_MAXCNT);
                    MAXCNT { MAXCNT }
                }
                TXD_AMOUNT {
                    TXD_AMOUNT($spim_ns TXD_AMOUNT);
                    AMOUNT { AMOUNT }
                }
                CONFIG {
                    CONFIG$(($uarte_ns CONFIG))*;
                    HWFC { HWFC }
                    PARITY { PARITY }
                    STOP { STOP }
                }
            }
        }
    };
}

#[cfg(nrf_mcu = "nrf9160")]
map_uarte_ns_s! {
    "Extracts UARTE0_NS register tokens.",
    "Extracts UARTE0_S register tokens.",
    periph_uarte0_ns,
    periph_uarte0_s,
    "UARTE0_NS peripheral variant.",
    "UARTE0_S peripheral variant.",
    Uarte0Ns,
    Uarte0S,
    UARTE0_NS,
    UARTE0_S,
    SPIM0_NS,
    SPIS0_NS,
    TWIM0_NS,
}

#[cfg(nrf_mcu = "nrf9160")]
map_uarte_ns_s! {
    "Extracts UARTE1_NS register tokens.",
    "Extracts UARTE1_S register tokens.",
    periph_uarte1_ns,
    periph_uarte1_s,
    "UARTE1_NS peripheral variant.",
    "UARTE1_S peripheral variant.",
    Uarte1Ns,
    Uarte1S,
    UARTE1_NS,
    UARTE1_S,
    SPIM1_NS,
    SPIS1_NS,
    TWIM1_NS,
}

#[cfg(nrf_mcu = "nrf9160")]
map_uarte_ns_s! {
    "Extracts UARTE2_NS register tokens.",
    "Extracts UARTE2_S register tokens.",
    periph_uarte2_ns,
    periph_uarte2_s,
    "UARTE2_NS peripheral variant.",
    "UARTE2_S peripheral variant.",
    Uarte2Ns,
    Uarte2S,
    UARTE2_NS,
    UARTE2_S,
    SPIM2_NS,
    SPIS2_NS,
    TWIM2_NS,
}

#[cfg(nrf_mcu = "nrf9160")]
map_uarte_ns_s! {
    "Extracts UARTE3_NS register tokens.",
    "Extracts UARTE3_S register tokens.",
    periph_uarte3_ns,
    periph_uarte3_s,
    "UARTE3_NS peripheral variant.",
    "UARTE3_S peripheral variant.",
    Uarte3Ns,
    Uarte3S,
    UARTE3_NS,
    UARTE3_S,
    SPIM3_NS,
    SPIS3_NS,
    TWIM3_NS,
}
