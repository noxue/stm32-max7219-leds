#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer, time::Hz};


// 初始化max7219
macro_rules! init_max7219 {
    ($clk:expr, $cs:expr, $din:expr) => {
        //0x00-0x08 设置为0
        for i in 0..=8 {
            write_max7219!($clk, $cs, $din, i, 0x00);
        }
        write_max7219!($clk, $cs, $din, 0x09, 0x00);
        write_max7219!($clk, $cs, $din, 0x0a, 0x05);
        write_max7219!($clk, $cs, $din, 0x0b, 0x07);
        write_max7219!($clk, $cs, $din, 0x0c, 0x01);
        write_max7219!($clk, $cs, $din, 0x0f, 0x00);
    };
}

macro_rules! write_max7219 {
    ($clk:expr, $cs:expr, $din:expr, $addr:expr, $data:expr) => {
        $cs.set_low();
        for i in 0..8 {
            $clk.set_low();
            if $addr << i & 0x80 == 0x80 {
                $din.set_high();
            } else {
                $din.set_low();
            }
            $clk.set_high();
        }
        
        for i in 0..8 {
            $clk.set_low();
            if $data << i & 0x80 == 0x80 {
                $din.set_high();
            } else {
                $din.set_low();
            }
            $clk.set_high();
        }
        $cs.set_high();
    };
}

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();
    
    


    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

// 指定时钟频率

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split();
    let mut gpiob = dp.GPIOB.split();

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(20.Hz()).unwrap();

    // // 电路接线
    // // clk  pb5
    // // cs   pb6
    // // din  pb7

    let mut clk = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);
    let mut cs = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);
    let mut din = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);

    // block!(timer.wait()).unwrap();

    // 初始化max7219
    init_max7219!(clk, cs, din);

    let data = [
        0x00, 0x00, 0x7C, 0xC6, 0xC6, 0xCE, 0xD6, 0xD6, // -0-.
        0xE6, 0xC6, 0xC6, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x38, 0x78, 0x18, 0x18,
        0x18, // -1-
        0x18, 0x18, 0x18, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7C, 0xC6, 0x06, 0x0C, 0x18,
        0x30, // -2-
        0x60, 0xC0, 0xC6, 0xFE, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7C, 0xC6, 0x06, 0x06, 0x3C,
        0x06, // -3-
        0x06, 0x06, 0xC6, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x1C, 0x3C, 0x6C, 0xCC,
        0xFE, // -4-
        0x0C, 0x0C, 0x0C, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFE, 0xC0, 0xC0, 0xC0, 0xFC,
        0x0E, // -5-
        0x06, 0x06, 0xC6, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x60, 0xC0, 0xC0, 0xFC,
        0xC6, // -6-
        0xC6, 0xC6, 0xC6, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFE, 0xC6, 0x06, 0x06, 0x0C,
        0x18, // -7-
        0x30, 0x30, 0x30, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7C, 0xC6, 0xC6, 0xC6, 0x7C,
        0xC6, // -8-
        0xC6, 0xC6, 0xC6, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7C, 0xC6, 0xC6, 0xC6, 0x7E,
        0x06, // -9-
        0x06, 0x06, 0x0C, 0x78, 0x00, 0x00, 0x00, 0x00u8,
    ];
    // Wait for the timer to trigger an update and change the state of the LED

    let size = data.len();
    let mut pos = 0;
    let mut j = 0;

    loop {
        
        for i in 0..8usize {
            j = (pos + i) % size;
            write_max7219!(clk, cs, din, (i + 1) as u8, data[j]);
        }
        pos = (pos + 1) % size;

        block!(timer.wait()).unwrap();

        // block!(timer.wait()).unwrap();
        // led.set_high();
        // block!(timer.wait()).unwrap();
        // led.set_low();
    }
}
