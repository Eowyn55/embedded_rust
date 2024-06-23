#![no_std]
#![no_main]

use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use embassy_rp::peripherals::PIN_25;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn blinker(mut led: Output<'static, PIN_25>, interval: Duration) {
    loop {
        led.set_high();
        Timer::after(interval).await;
        led.set_low();
        Timer::after(interval).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // todo: check the pin!
    let led = Output::new(p.PIN_25, Level::Low);
    unwrap!(spawner.spawn(blinker(led, Duration::from_millis(300))));
}
