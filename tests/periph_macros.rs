use drone_core::token::Token;
use drone_nrf_map::nrf_reg_tokens;

nrf_reg_tokens! {
    struct Regs;
}

#[test]
#[allow(unused_variables)]
fn periph_macros8() {
    let reg = unsafe { Regs::take() };
    #[cfg(nrf_mcu = "nrf9160")]
    {
        let uarte0 = drone_nrf_map::periph::uarte::periph_uarte0_ns!(reg);
    }
}
