use drone_core::token::Token;
use drone_nrf_map::nrf_reg_tokens;

nrf_reg_tokens! {
    index => pub Regs;
}

#[test]
#[allow(unused_variables)]
fn periph_macros8_ns() {
    let reg = unsafe { Regs::take() };
    #[cfg(all(feature = "uarte", nrf_mcu = "nrf9160"))]
    {
        let uarte0 = drone_nrf_map::periph::uarte::periph_uarte0_ns!(reg);
    }
}

#[test]
#[allow(unused_variables)]
fn periph_macros8_s() {
    let reg = unsafe { Regs::take() };
    #[cfg(all(feature = "uarte", nrf_mcu = "nrf9160"))]
    {
        let uarte0 = drone_nrf_map::periph::uarte::periph_uarte0_s!(reg);
    }
}
