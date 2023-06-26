#![feature(prelude_import)]
#![no_std]
#![no_main]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1::stm32f103;
#[doc(hidden)]
#[export_name = "main"]
pub unsafe extern "C" fn __cortex_m_rt_main_trampoline() {
    __cortex_m_rt_main()
}
fn __cortex_m_rt_main() -> ! {
    let peripherals = stm32f103::Peripherals::take().unwrap();
    let gpioc = &peripherals.GPIOC;
    let gpiob = &peripherals.GPIOB;
    let rcc = &peripherals.RCC;
    rcc.apb2enr.write(|w| unsafe { w.bits(0xFFFFFFFF) });
    gpioc
        .crh
        .write(|w| {
            w.mode13().bits(0b11);
            w.cnf13().bits(0b00)
        });
    gpiob.crl.write(|w| unsafe { w.bits(0b0011_0011_0011_0000_0000_0000_0000_0000) });
    gpiob
        .crh
        .write(|w| {
            w.mode8().bits(0b11);
            w.cnf8().bits(0b00)
        });
    let data = 0x00;
    for i in 0..=8 {
        let gpiob = gpiob;
        gpiob.bsrr.write(|w| w.br6().set_bit());
        let data = i;
        for i in 0..8 {
            gpiob.bsrr.write(|w| w.br5().set_bit());
            if data << i & 0x01 == 1 {
                gpiob.bsrr.write(|w| w.bs7().set_bit());
            } else {
                gpiob.bsrr.write(|w| w.br7().set_bit());
            }
            gpiob.bsrr.write(|w| w.bs5().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs6().set_bit());
        gpiob.bsrr.write(|w| w.br6().set_bit());
        let data = 0x0;
        for i in 0..8 {
            gpiob.bsrr.write(|w| w.br5().set_bit());
            if data << i & 0x01 == 1 {
                gpiob.bsrr.write(|w| w.bs7().set_bit());
            } else {
                gpiob.bsrr.write(|w| w.br7().set_bit());
            }
            gpiob.bsrr.write(|w| w.bs5().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs6().set_bit());
    }
    let gpiob = gpiob;
    gpiob.bsrr.write(|w| w.br6().set_bit());
    let data = 0x9;
    for i in 0..8 {
        gpiob.bsrr.write(|w| w.br5().set_bit());
        if data << i & 0x01 == 1 {
            gpiob.bsrr.write(|w| w.bs7().set_bit());
        } else {
            gpiob.bsrr.write(|w| w.br7().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs5().set_bit());
    }
    gpiob.bsrr.write(|w| w.bs6().set_bit());
    gpiob.bsrr.write(|w| w.br6().set_bit());
    let data = 0x0;
    for i in 0..8 {
        gpiob.bsrr.write(|w| w.br5().set_bit());
        if data << i & 0x01 == 1 {
            gpiob.bsrr.write(|w| w.bs7().set_bit());
        } else {
            gpiob.bsrr.write(|w| w.br7().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs5().set_bit());
    }
    gpiob.bsrr.write(|w| w.bs6().set_bit());
    let gpiob = gpiob;
    gpiob.bsrr.write(|w| w.br6().set_bit());
    let data = 0xA;
    for i in 0..8 {
        gpiob.bsrr.write(|w| w.br5().set_bit());
        if data << i & 0x01 == 1 {
            gpiob.bsrr.write(|w| w.bs7().set_bit());
        } else {
            gpiob.bsrr.write(|w| w.br7().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs5().set_bit());
    }
    gpiob.bsrr.write(|w| w.bs6().set_bit());
    gpiob.bsrr.write(|w| w.br6().set_bit());
    let data = 0x0;
    for i in 0..8 {
        gpiob.bsrr.write(|w| w.br5().set_bit());
        if data << i & 0x01 == 1 {
            gpiob.bsrr.write(|w| w.bs7().set_bit());
        } else {
            gpiob.bsrr.write(|w| w.br7().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs5().set_bit());
    }
    gpiob.bsrr.write(|w| w.bs6().set_bit());
    let gpiob = gpiob;
    gpiob.bsrr.write(|w| w.br6().set_bit());
    let data = 0xB;
    for i in 0..8 {
        gpiob.bsrr.write(|w| w.br5().set_bit());
        if data << i & 0x01 == 1 {
            gpiob.bsrr.write(|w| w.bs7().set_bit());
        } else {
            gpiob.bsrr.write(|w| w.br7().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs5().set_bit());
    }
    gpiob.bsrr.write(|w| w.bs6().set_bit());
    gpiob.bsrr.write(|w| w.br6().set_bit());
    let data = 0x7;
    for i in 0..8 {
        gpiob.bsrr.write(|w| w.br5().set_bit());
        if data << i & 0x01 == 1 {
            gpiob.bsrr.write(|w| w.bs7().set_bit());
        } else {
            gpiob.bsrr.write(|w| w.br7().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs5().set_bit());
    }
    gpiob.bsrr.write(|w| w.bs6().set_bit());
    let gpiob = gpiob;
    gpiob.bsrr.write(|w| w.br6().set_bit());
    let data = 0xC;
    for i in 0..8 {
        gpiob.bsrr.write(|w| w.br5().set_bit());
        if data << i & 0x01 == 1 {
            gpiob.bsrr.write(|w| w.bs7().set_bit());
        } else {
            gpiob.bsrr.write(|w| w.br7().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs5().set_bit());
    }
    gpiob.bsrr.write(|w| w.bs6().set_bit());
    gpiob.bsrr.write(|w| w.br6().set_bit());
    let data = 0x1;
    for i in 0..8 {
        gpiob.bsrr.write(|w| w.br5().set_bit());
        if data << i & 0x01 == 1 {
            gpiob.bsrr.write(|w| w.bs7().set_bit());
        } else {
            gpiob.bsrr.write(|w| w.br7().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs5().set_bit());
    }
    gpiob.bsrr.write(|w| w.bs6().set_bit());
    let gpiob = gpiob;
    gpiob.bsrr.write(|w| w.br6().set_bit());
    let data = 0xF;
    for i in 0..8 {
        gpiob.bsrr.write(|w| w.br5().set_bit());
        if data << i & 0x01 == 1 {
            gpiob.bsrr.write(|w| w.bs7().set_bit());
        } else {
            gpiob.bsrr.write(|w| w.br7().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs5().set_bit());
    }
    gpiob.bsrr.write(|w| w.bs6().set_bit());
    gpiob.bsrr.write(|w| w.br6().set_bit());
    let data = 0x0;
    for i in 0..8 {
        gpiob.bsrr.write(|w| w.br5().set_bit());
        if data << i & 0x01 == 1 {
            gpiob.bsrr.write(|w| w.bs7().set_bit());
        } else {
            gpiob.bsrr.write(|w| w.br7().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs5().set_bit());
    }
    gpiob.bsrr.write(|w| w.bs6().set_bit());
    let gpiob = gpiob;
    gpiob.bsrr.write(|w| w.br6().set_bit());
    let data = 0x1;
    for i in 0..8 {
        gpiob.bsrr.write(|w| w.br5().set_bit());
        if data << i & 0x01 == 1 {
            gpiob.bsrr.write(|w| w.bs7().set_bit());
        } else {
            gpiob.bsrr.write(|w| w.br7().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs5().set_bit());
    }
    gpiob.bsrr.write(|w| w.bs6().set_bit());
    gpiob.bsrr.write(|w| w.br6().set_bit());
    let data = 0xFF;
    for i in 0..8 {
        gpiob.bsrr.write(|w| w.br5().set_bit());
        if data << i & 0x01 == 1 {
            gpiob.bsrr.write(|w| w.bs7().set_bit());
        } else {
            gpiob.bsrr.write(|w| w.br7().set_bit());
        }
        gpiob.bsrr.write(|w| w.bs5().set_bit());
    }
    gpiob.bsrr.write(|w| w.bs6().set_bit());
    loop {}
}
