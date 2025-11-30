#![no_std]
#![no_main]

use {
    cortex_m as _, cortex_m_rt as _, defmt_rtt as _,
    embassy_stm32::gpio::{Level, Speed},
    panic_probe as _,
};

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let config = embassy_stm32::Config::default();
    let p = embassy_stm32::init(config);

    let mut ld4 = embassy_stm32::gpio::Output::new(p.PC7, Level::Low, Speed::Low);
    ld4.set_high();

    loop {
        embassy_time::Timer::after_secs(1).await;
    }
}
