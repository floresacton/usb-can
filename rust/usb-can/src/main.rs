#![no_std]
#![no_main]

use panic_halt as _;

use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use stm32g4::stm32g473;

static MILLIS: AtomicU32 = AtomicU32::new(0);

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32g473::Peripherals::take().unwrap();

    dp.RCC.cr().modify(|_, w| w.hseon().set_bit());
    while dp.RCC.cr().read().hserdy().bit_is_clear() {}

    // this is outside spec, should be 4 but works for now
    dp.FLASH.acr().modify(|_, w| w.latency().wait2());

    // if low power needed, configure power stuff here

    dp.RCC.pllcfgr().modify(|_, w| {
        w.pllsrc().hse();
        w.pllm().div1();
        w.plln().div18();
        w.pllr().div2();
        w.pllren().set_bit()
    });

    dp.RCC.cr().modify(|_, w| w.pllon().set_bit());
    while dp.RCC.cr().read().pllrdy().bit_is_clear() {}

    dp.RCC.cfgr().modify(|_, w| w.sw().pll());
    while !dp.RCC.cfgr().read().sws().is_pll() {}

    cp.SYST.set_clock_source(SystClkSource::Core);
    cp.SYST.set_reload(144_000_000 / 1000 - 1);
    cp.SYST.clear_current();
    cp.SYST.enable_interrupt();
    cp.SYST.enable_counter();

    dp.RCC.ahb2enr().modify(|_, w| w.gpioaen().set_bit());

    dp.GPIOA.moder().modify(|_, w| w.moder2().output());
    //dp.GPIOA.otyper().modify(|_, w| w.ot2().push_pull());
    //dp.GPIOA.ospeedr().modify(|_, w| w.ospeedr2().low_speed());
    //dp.GPIOA.pupdr().modify(|_, w| w.pupdr2().floating());

    loop {
        dp.GPIOA.bsrr().write(|w| w.bs2().set_bit());
        delay_ms(1000);

        dp.GPIOA.bsrr().write(|w| w.br2().set_bit());
        delay_ms(1000);
    }
}

#[exception]
fn SysTick() {
    MILLIS.fetch_add(1, Ordering::Relaxed);
}

fn millis() -> u32 {
    MILLIS.load(Ordering::Relaxed)
}

fn delay_ms(ms: u32) {
    let start = millis();
    while millis() - start < ms {}
}
