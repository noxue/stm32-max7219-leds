#![no_std]
#![no_main]

// pick a panicking behavior
use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1::stm32f103;

macro_rules! write_max7219_byte {
    ($io:expr, $data:expr) => {
        let gpiob = $io;
        gpiob.bsrr.write(|w| w.br6().set_bit());
        let data = $data;
        for i in 0..8 {
            // CLK = 0;
            gpiob.bsrr.write(|w| w.br5().set_bit());
            if data << i & 0x80 == 1 {
                // DIN = 1;
                gpiob.bsrr.write(|w| w.bs7().set_bit());
            } else {
                // DIN = 0;
                gpiob.bsrr.write(|w| w.br7().set_bit());
            }
            // CLK = 1;
            gpiob.bsrr.write(|w| w.bs5().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs6().set_bit());
    };
}

macro_rules! write_max7219 {
    ($gpio:expr,$addr:expr,$data:expr) => {
        // cs = 0;
        let gpiob = $gpio;
        gpiob.bsrr.write(|w| w.br6().set_bit());
        write_max7219_byte!($gpio, $addr);
        write_max7219_byte!($gpio, $data);
        // cs = 1;
        gpiob.bsrr.write(|w| w.bs6().set_bit());
    };
}

// 电路接线
// clk  pb5
// cs   pb6
// din  pb7

#[entry]
fn main() -> ! {
    let peripherals = stm32f103::Peripherals::take().unwrap();
    let gpioc = &peripherals.GPIOC;
    let gpiob = &peripherals.GPIOB;

    let rcc = &peripherals.RCC;

    // 允许所有gpio口时钟
    rcc.apb2enr.write(|w| {
        w.iopcen().set_bit();
        w.iopben().set_bit()
    });

    gpioc.crh.write(|w| {
        w.mode13().bits(0b11);
        w.cnf13().bits(0b00)
    });

    gpiob
        .crl
        .write(|w| unsafe { w.bits(0b0011_0011_0011_0000_0000_0000_0000_0000) });

    for i in 0..9 {
        write_max7219!(gpiob, i, 0x00);
    }
    write_max7219!(gpiob, 0x09, 0x00); //译码方式：BCD码
    write_max7219!(gpiob, 0x0a, 0x03); //亮度
    write_max7219!(gpiob, 0x0b, 0x07); //扫描界限；8个数码管显示
    write_max7219!(gpiob, 0x0c, 0x01); //掉电模式：0，普通模式：1
    write_max7219!(gpiob, 0x0f, 0x01); //显示测试：1；测试结束，正常显示：0
                                       // write_max7219!(gpiob, 0x01, 0xff);

    loop {
        for i in 1..9 {
            write_max7219!(gpiob, i, 0x00);
            asm::delay(1000000);
        }
    }
}
