#![no_std]
#![no_main]

use {
    cortex_m as _, cortex_m_rt as _, defmt_rtt as _,
    embassy_stm32::{
        bind_interrupts,
        gpio::{Level, Speed},
        pac,
        usart::{DataBits, Parity, StopBits},
    },
    panic_probe as _,
    static_cell::StaticCell,
};

bind_interrupts!(struct Usart2Irq {
    USART2 => embassy_stm32::usart::InterruptHandler::<embassy_stm32::peripherals::USART2>;
});

#[cortex_m_rt::entry]
fn main() -> ! {
    // pac::FLASH.acr().modify(|reg| {
    //     reg.set_latency(pac::flash::vals::Latency::WS1);
    // });
    // pac::DBGMCU.cr().modify(|reg| {
    //     reg.set_dbg_standby(true);
    //     reg.set_dbg_stop(true);
    // });
    // pac::RCC.ahbenr().modify(|reg| {
    //     reg.set_dmaen(true);
    // });

    static EXECUTOR: StaticCell<embassy_executor::Executor> = StaticCell::new();
    let executor = EXECUTOR.init(embassy_executor::Executor::new());
    executor.run(|spawner| {
        spawner.must_spawn(main(spawner));
    });
}

#[embassy_executor::task]
// #[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    defmt::info!("initializing...");
    let mut config = embassy_stm32::Config::default();
    let p = embassy_stm32::init(config);

    let mut ld4 = embassy_stm32::gpio::Output::new(p.PC7, Level::Low, Speed::Low);
    ld4.set_high();

    let mut usart2conf = embassy_stm32::usart::Config::default();
    usart2conf.data_bits = DataBits::DataBits8;
    usart2conf.parity = Parity::ParityNone;
    usart2conf.stop_bits = StopBits::STOP1;
    usart2conf.baudrate = 9600;

    let mut uart = defmt::unwrap!(embassy_stm32::usart::Uart::new(
        p.USART2, p.PA3, p.PA2, Usart2Irq, p.DMA1_CH4, p.DMA1_CH5, usart2conf
    ));

    let msg = b"Hello, from Rust!\n";

    loop {
        defmt::info!("Sending message...");
        defmt::unwrap!(uart.write(msg).await);
        embassy_time::Timer::after_secs(5).await;
    }
}
