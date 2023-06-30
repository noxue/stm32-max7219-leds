#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};


struct MatrixLed<
    const P_CLK: char,
    const N_CLK: u8,
    const P_CS: char,
    const N_CS: u8,
    const P_DIN: char,
    const N_DIN: u8,
> {
    clk: stm32f1xx_hal::gpio::Pin<P_CLK, N_CLK, stm32f1xx_hal::gpio::Output>,
    cs: stm32f1xx_hal::gpio::Pin<P_CS, N_CS, stm32f1xx_hal::gpio::Output>,
    din: stm32f1xx_hal::gpio::Pin<P_DIN, N_DIN, stm32f1xx_hal::gpio::Output>,
    blocks: u8,
}

impl<
        const P_CLK: char,
        const N_CLK: u8,
        const P_CS: char,
        const N_CS: u8,
        const P_DIN: char,
        const N_DIN: u8,
    > MatrixLed<P_CLK, N_CLK, P_CS, N_CS, P_DIN, N_DIN>
{
    pub fn new(
        clk: stm32f1xx_hal::gpio::Pin<P_CLK, N_CLK, stm32f1xx_hal::gpio::Output>,
        cs: stm32f1xx_hal::gpio::Pin<P_CS, N_CS, stm32f1xx_hal::gpio::Output>,
        din: stm32f1xx_hal::gpio::Pin<P_DIN, N_DIN, stm32f1xx_hal::gpio::Output>,
        blocks: u8, // 级联多少块
    ) -> Self {
        let mut m = MatrixLed {
            clk,
            cs,
            din,
            blocks,
        };
        m.init();
        m
    }

    pub fn write_byte(&mut self, data: u8) {
        for i in 0..8 {
            self.clk.set_low();
            if data << i & 0x80 == 0x80 {
                self.din.set_high();
            } else {
                self.din.set_low();
            }
            self.clk.set_high();
        }
    }

    pub fn write(&mut self, addr: u8, data: u8) {
        self.cs.set_low();
        self.write_byte(addr);
        self.write_byte(data);
        self.cs.set_high();
    }

    pub fn write_block(&mut self, block: u8, addr: u8, data: u8) {
        self.cs.set_low();

        self.write_byte(addr);
        self.write_byte(data);

        // 写到第 n 块 就写 n-1 次 16位数据，把前面的数据推到第 n 块 led上
        for _ in 1..block {
            self.write_byte(0x00);
            self.write_byte(0x00);
        }

        // 这句是最主要的 推完之后再上拉
        self.cs.set_high();
    }

    fn init(&mut self) {
        for i in 0..=8 {
            self.write(i, 0x00);
        }
        self.write(0x09, 0x00); // 译码方式：无译码
        self.write(0x0a, 0x03); // 亮度
        self.write(0x0b, 0x07); // 扫描界限；8个数码管显示
        self.write(0x0c, 0x01); // 使能显示
        self.write(0x0f, 0x00); // 显示测试：正常显示
    }

    // test on
    pub fn test(&mut self) {
        for i in 0..self.blocks {
            self.write_block(i, 0x0f, 0x01);
        }
    }

    // test off
    pub fn test_off(&mut self) {
        for i in 0..self.blocks {
            self.write_block(i, 0x0f, 0x00);
        }
    }
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

    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();

    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();

    let clk = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);
    let cs = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);
    let din = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);

    let mut leds = MatrixLed::new(clk, cs, din, 4);

    leds.test();
    block!(timer.wait()).unwrap();
    leds.test_off();

    loop {
        leds.write_block(3, 2, 3);
    }
}
